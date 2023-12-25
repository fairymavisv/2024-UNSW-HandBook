import { Injectable } from '@nestjs/common';
import { CourseInterface } from './course.interface';
import {InjectModel} from "@nestjs/mongoose";
import mongoose, {Model} from "mongoose";
import {Course, CourseInfo,Comment} from "./course.model";


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
                        rating: comment.rating,
                        username: (comment.userId as any).username // 使用类型断言
                    };
                }) : [] // 如果 courseFromMongo 为空，则返回空数组
            };
            return courseInfo;
        } else {
            throw new Error('Course not found');
        }
    }

    async createCourseComment(CourseCode: string, userId: string, text: string, rating: number) {
        let courseFromMongo = await this.courseModel.findOne({ courseCode: CourseCode });
        console.log("courseFromMongo", courseFromMongo);

        if(!courseFromMongo) {
            const newCourse = new this.courseModel({
                courseCode: CourseCode,
                comments: []
            });
            await newCourse.save();

        }
        courseFromMongo = await this.courseModel.findOne({ courseCode: CourseCode });
        console.log("courseFromMongo after create", courseFromMongo);

        // 确保这里使用的是针对评论的模型
        const newComment = new this.commentModel({
            text: text,
            userId: userId, // 如果 userId 已经是字符串形式的 ObjectId，则无需转换
            updatedAt: new Date(),
            rating: rating
        });
        console.log("newComment", newComment);
        courseFromMongo.comments.push(newComment);
        await courseFromMongo.save();
        console.log("courseFromMongo after save", courseFromMongo);
        return newComment;
    }

}
