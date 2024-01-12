
import mongoose from 'mongoose';
//从接口获取的课程基本信息
export interface BasicCourseInfo {
    code: string;
    name: string;
    UOC: number;
    description: string;
    conditions: string[];
    offerterms: string[];
}
//前端展示的格式,用 nickname 替代 username
export interface ExtendedComment {
    text: string;      // 评论文本
    updatedAt: Date;   // 最后修改时间
    difficulty: number;    // 难度评分
    usefulness: number;    // 有用程度评分
    workload: number;    // 工作量评分
    nickname : string; // 关联的用户名
}
//储存在数据库中的评论格式
export interface Comment extends mongoose.Document {
    text: string;      // 评论文本
    username: string; // 用户ID
    updatedAt: Date;   // 最后一次修改时间
    difficulty: number;    // 难度评分
    usefulness: number;    // 有用程度评分
    workload: number;    // 工作量评分
}
//课程信息
export interface CourseInfo {
    basicInfo: BasicCourseInfo; // 从 CourseInterface 获取的基本信息
    comments: ExtendedComment[]; // 从 MongoDB 获取的评论
}

//数据库中的课程格式
export interface Course extends mongoose.Document {
    courseCode: string;
    comments: Comment[];

}
//数据库中评论的格式
export const commentSchema = new mongoose.Schema({
    text: { type: String, required: true },
    username: { type: String},
    updatedAt: { type: Date, default: Date.now }, // 保留最后一次修改的时间
    difficulty: { type: Number, required: true },
    usefulness: { type: Number, required: true },
    workload: { type: Number, required: true },
},{collection: 'course'});

commentSchema.index({ updatedAt: -1 }); // -1 表示降序索引


export const courseSchema = new mongoose.Schema({
    courseCode: String,
    comments: [commentSchema], // 嵌入评论
},{collection: 'course'} ); //use to connect to the collection in the database

