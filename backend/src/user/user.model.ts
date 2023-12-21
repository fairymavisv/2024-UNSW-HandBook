
import mongoose from 'mongoose';

export interface User extends mongoose.Document {
    // ...其他字段
    username: string;
    programs: string;
    courseslist: string[];
}

export const userSchema = new mongoose.Schema({
    // ...其他字段定义
    username: { type: String, required: true },
    password: { type: String, required: true },
    programs: { type: String },
    courseslist: { type: [String], default: [] }
}, { collection: 'user' });

//在这个 Schema 定义中，通过 { collection: 'user' } 明确指定集合名为 user。


