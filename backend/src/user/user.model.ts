
import mongoose from 'mongoose';

export interface User extends mongoose.Document {
    // ...其他字段
    username: string;
    password: string;
    nickname: string;
    programs: string;
    courseslist: string[];
}

export const userSchema = new mongoose.Schema({
    // ...其他字段定义
    username: { type: String, required: true, unique: true },
    password: { type: String, required: true },
    nickname: { type: String },
    programs: { type: String },
    courseslist: { type: [String], default: [] }
}, { collection: 'user' });

//在这个 Schema 定义中，通过 { collection: 'user' } 明确指定集合名为 user。

export const userModel = mongoose.model<User>('User', userSchema);
