import {HttpException, HttpStatus, Injectable} from '@nestjs/common';
import { CourseInterface } from './course.interface';
import {InjectModel} from "@nestjs/mongoose";
import mongoose, {Model} from "mongoose";
import {Course, CourseInfo,Comment} from "./course.model";
import {CreateCommentDto} from "./course.dto";



@Injectable()
export class CourseService {
    constructor(
        @InjectModel('Course') private courseModel: Model<Course>,
        @InjectModel('Comment') private commentModel: Model<Comment>
    ) {}

    async getCourseInfo(CourseCode: string): Promise<CourseInfo> {
        const course = await CourseInterface.getCourseInfo(CourseCode);
        // 使用 populate 获取评论相关联的用户信息
        const courseFromMongo = await this.courseModel.findOne({ courseCode: CourseCode })
            .populate({
                path: 'comments.userId', // 指定要填充的字段
                model: 'User', // 指定关联的模型
                select: 'username' // 仅选择 username 字段
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
                        userId: comment.userId,
                        updatedAt: comment.updatedAt,
                        username: (comment.userId as any).username, // 使用类型断言
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

    // async createCourseComment(CourseCode: string, userId: string, text: string, rating: number) {
    //
    //     const course = await CourseInterface.getCourseInfo(CourseCode);
    //     if (!course) {
    //         throw new Error('Course not found');
    //     }
    //     let courseFromMongo = await this.courseModel.findOne({ courseCode: CourseCode });
    //     console.log("courseFromMongo", courseFromMongo);
    //
    //     if(!courseFromMongo) {
    //         const newCourse = new this.courseModel({
    //             courseCode: CourseCode,
    //             comments: []
    //         });
    //         await newCourse.save();
    //
    //     }
    //     courseFromMongo = await this.courseModel.findOne({ courseCode: CourseCode });
    //     console.log("courseFromMongo after create", courseFromMongo);
    //
    //     // 确保这里使用的是针对评论的模型
    //     const newComment = new this.commentModel({
    //         text: text,
    //         userId: userId, // 如果 userId 已经是字符串形式的 ObjectId，则无需转换
    //         updatedAt: new Date(),
    //         rating: rating
    //     });
    //     console.log("newComment", newComment);
    //     courseFromMongo.comments.push(newComment);
    //     await courseFromMongo.save();
    //     console.log("courseFromMongo after save", courseFromMongo);
    //     return newComment;
    // }

    async createCourseComment(createCommentDto: CreateCommentDto) {
        const { courseCode, userId, text, difficulty, usefulness, workload } = createCommentDto;
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
            userId: userId, // Assuming userId is already a string ObjectId
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

    async deleteCourseComment(deleteCommentID: string) {
        console.log("deleteCommentID", deleteCommentID);
        // 假设 courseModel 是指向课程集合的模型
        const result = await this.courseModel.updateMany(
            { "comments._id": deleteCommentID },
            { $pull: { comments: { _id: deleteCommentID } } }
        );

        if (result.modifiedCount === 0) {
            throw new Error('Course comment not found or already deleted');
        }

        return deleteCommentID + " has been deleted";

    }
}
