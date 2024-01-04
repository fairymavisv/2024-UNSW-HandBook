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
        const courseFromMongo = await this.courseModel.findOne({ courseCode: CourseCode })
            .populate({
                path: 'comments.username', // 指定要填充的字段
                model: 'User', // 指定关联的模型
                select: 'nickname' // 仅选择 username 字段
            });

        console.log("CourseCode:", CourseCode);
        console.log("courseFromMongo", courseFromMongo);
        console.log("course", course);

        if (course) {
            const courseInfo: CourseInfo = {
                basicInfo: course,
                comments: courseFromMongo ? courseFromMongo.comments.map(comment => {
                    return {
                        text: comment.text,
                        updatedAt: comment.updatedAt,
                        nickname: (comment.username as any).nickname, // 使用类型断言
                        difficulty: comment.difficulty,
                        usefulness: comment.usefulness,
                        workload: comment.workload
                    };
                }) : [] // 如果 courseFromMongo 为空，则返回空数组
            };
            return courseInfo;
        } else {
            throw new Error('Course not found');
        }
    }


    async createCourseComment(createCommentDto: CreateCommentDto) {
        const { courseCode, token, text, difficulty, usefulness, workload } = createCommentDto;

        const username = await this.jwtService.verifyToken(token);
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
        const username = await this.jwtService.verifyToken(token);

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
}
