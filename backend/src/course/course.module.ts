import { Module } from '@nestjs/common';
import { CourseController } from './course.controller';
import { CourseService } from './course.service';
import { MongooseModule } from '@nestjs/mongoose';
import {ConfigModule} from "@nestjs/config";
import {commentSchema, courseSchema} from './course.model';


@Module({

    imports: [
        ConfigModule.forRoot({
            isGlobal: true,
        }),
        MongooseModule.forFeature([{ name: 'Course', schema: courseSchema }, { name: 'Comment', schema: commentSchema }]),
        MongooseModule.forRoot(process.env.MONGO_URI),
    ],
    controllers: [CourseController],
    providers: [CourseService],
})
export class CourseModule {}
