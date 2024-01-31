import {Injectable} from '@nestjs/common';
import { CourseInterface } from './course.interface';
import {InjectModel} from "@nestjs/mongoose";
import mongoose, {Model} from "mongoose";
import {Course, CourseInfo,Comment} from "./course.model";
import {CreateCommentDto, DeleteCommentDto} from "./course.dto";
import {User} from "../user/user.model";
import {JwtAuthService} from "../jwt.service";



@Injectable()
export class CourseService {
    constructor(
        @InjectModel('Course') private courseModel: Model<Course>,
        @InjectModel('Comment') private commentModel: Model<Comment>,
        @InjectModel('User') private userModel: Model<User>, private jwtService: JwtAuthService

    ) {}

    async getCourseInfo(CourseCode: string): Promise<CourseInfo> {
        const course = await CourseInterface.getCourseInfo(CourseCode);
        // 使用 populate 获取评论相关联的用户信息
        const courseFromMongo = await this.courseModel.findOne({ courseCode: CourseCode });

        console.log("CourseCode:", CourseCode);
        console.log("courseFromMongo", courseFromMongo);
        console.log("course", course);
        let commentsWithNicknames = [];
        if (course) {
            if (courseFromMongo) {
                commentsWithNicknames = await Promise.all(courseFromMongo.comments.map(async (comment) => {
                    const user = await this.userModel.findOne({ username: comment.username }).select('nickname');

                    return {
                        text: comment.text,
                        updatedAt: comment.updatedAt,
                        nickname: user ? user.nickname : null, // 如果找到用户，则返回昵称，否则返回 null
                        difficulty: comment.difficulty,
                        usefulness: comment.usefulness,
                        workload: comment.workload
                    };
                }));
            }else{
                const upperCaseCode = CourseCode.toUpperCase();
                const newCourse = new this.courseModel({
                    courseCode: upperCaseCode,
                    comments: []
                });
                await newCourse.save();
            }
            const courseInfo: CourseInfo = {
                basicInfo: course,
                comments: commentsWithNicknames
            };
            return courseInfo;
        } else {
            throw new Error('Course not found');
        }
    }




    async createCourseComment(createCommentDto: CreateCommentDto) {
        const { courseCode, token, text, difficulty, usefulness, workload } = createCommentDto;

        const username = await this.jwtService.verifyToken(token, 'access')
        // 验证用户是否有权限评论
        const userfromMoogo = await this.userModel.findOne({username: username});
        if (!userfromMoogo) {
            throw new Error("Authentication failed");
        }

        const upperCaseCode = courseCode.toUpperCase();

        const course = await CourseInterface.getCourseInfo(upperCaseCode);
        if (!course) {
            throw new Error('Course not found');
        }
        let courseFromMongo = await this.courseModel.findOne({ courseCode: upperCaseCode });
        console.log("courseFromMongo", courseFromMongo);

        if(!courseFromMongo) {
            const newCourse = new this.courseModel({
                courseCode: upperCaseCode,
                comments: []
            });
            await newCourse.save();

        }
        courseFromMongo = await this.courseModel.findOne({ courseCode: upperCaseCode });
        console.log("courseFromMongo after create", courseFromMongo);
        // 验证必需字段
        if (difficulty === undefined || usefulness === undefined || workload === undefined) {
            throw new Error('缺少必要的评分维度');
        }
        // Create a new comment
        const newComment = new this.commentModel({
            text: text,
            username: username, // Assuming username is already a string ObjectId
            updatedAt: new Date(),
            difficulty: difficulty,
            usefulness: usefulness,
            workload: workload
        });

        // Add comment to the course
        courseFromMongo.comments.push(newComment);

        // Save the updated course
        await courseFromMongo.save();

        return newComment;
    }


    //TODO：是否需要更改评论

    async deleteCourseComment(deleteCommentDto: DeleteCommentDto) {
        console.log("deleteCommentID", deleteCommentDto);
        const { commentID, token } = deleteCommentDto;
        const username = await this.jwtService.verifyToken(token, 'access');

        // 验证用户是否有权限删除评论
        const userfromMoogo = await this.userModel.findOne({username: username});
        if (!userfromMoogo) {
            throw new Error("Authentication failed");
        }

        // 假设 courseModel 是指向课程集合的模型
        const result = await this.courseModel.updateMany(
            { "comments._id": commentID },
            { $pull: { comments: { _id: commentID } } }
        );

        if (result.modifiedCount === 0) {
            throw new Error('Course comment not found or already deleted');
        }

        return commentID + " has been deleted";

    }

    async getRecommendCourse(){
        //遍历mongoDB中所有的course，对于每一个course，算出它们的comment的difficulty，usefulness，workload的平均值
        //然后根据这些平均值进行排序，返回前十个
        const courses = await this.courseModel.find();
        let courseList = [];
        for(let i = 0; i < courses.length; i++){
            let course = courses[i];
            let difficulty = 0;
            let usefulness = 0;
            let workload = 0;
            for(let j = 0; j < course.comments.length; j++){
                let comment = course.comments[j];
                difficulty += comment.difficulty;
                usefulness += comment.usefulness;
                workload += comment.workload;
            }
            difficulty /= course.comments.length;
            usefulness /= course.comments.length;
            workload /= course.comments.length;
            courseList.push({
                courseCode: course.courseCode,
                difficulty: difficulty,
                usefulness: usefulness,
                workload: workload
            });
        }

        //在courseList中，优先按照usefulness从大到小排序，如果usefulness相同，按照workload从小到大排序,如果workload相同，按照difficulty从小到大排序
        courseList.sort(function(a, b){
            if(a.usefulness !== b.usefulness){
                return b.usefulness - a.usefulness;
            }
            else if(a.workload !== b.workload){
                return a.workload - b.workload;
            }
            else{
                return a.difficulty - b.difficulty;
            }
        });

        //返回前5个,包括它们的三项平均值
        let ret = [];
        for(let i = 0; i < 5; i++){
            let course = courseList[i];
            ret.push({
                courseCode: course.courseCode,
                difficulty: course.difficulty,
                usefulness: course.usefulness,
                workload: course.workload
            });
        }
        return ret;
    }
}
