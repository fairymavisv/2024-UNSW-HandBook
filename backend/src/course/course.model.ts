
import mongoose from 'mongoose';

export interface BasicCourseInfo {
    code: string;
    name: string;
    UOC: number;
    description: string;
    conditions: string[];
    offerterms: string[];
}

export interface ExtendedComment {
    text: string;      // 评论文本
    userId: string | mongoose.Schema.Types.ObjectId; // 用户ID
    updatedAt: Date;   // 最后修改时间
    rating: number;    // 评分
    username?: string; // 关联的用户名
}

export interface Comment extends mongoose.Document {
    text: string;      // 评论文本
    rating: number;    // 评分
    userId: mongoose.Schema.Types.ObjectId; // 用户ID
    updatedAt: Date;   // 最后一次修改时间
}

export interface CourseInfo {
    basicInfo: BasicCourseInfo; // 从 CourseInterface 获取的基本信息
    comments: ExtendedComment[]; // 从 MongoDB 获取的评论
}


export interface Course extends mongoose.Document {
    courseCode: string;
    comments: Comment[];

}

const commentSchema = new mongoose.Schema({
    text: { type: String, required: true },
    userId: { type: mongoose.Schema.Types.ObjectId, ref: 'User' },
    updatedAt: { type: Date, default: Date.now }, // 保留最后一次修改的时间
    rating: { type: Number, min: 0, max: 5, required: true },
});

commentSchema.index({ updatedAt: -1 }); // -1 表示降序索引


export const courseSchema = new mongoose.Schema({
    courseCode: String,
    comments: [commentSchema], // 嵌入评论
},{collection: 'course'} ); //use to connect to the collection in the database

