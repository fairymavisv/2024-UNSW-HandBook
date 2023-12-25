import { Injectable } from '@nestjs/common';
import { CourseInterface } from './course.interface';
import {InjectModel} from "@nestjs/mongoose";
import {Model} from "mongoose";
import {Course, CourseInfo} from "./course.model";


@Injectable()
export class CourseService {
    constructor(
        @InjectModel('Course') private courseModel: Model<Course>
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
}
