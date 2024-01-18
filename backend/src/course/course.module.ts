import { Module } from '@nestjs/common';
import { CourseController } from './course.controller';
import { CourseService } from './course.service';
import { MongooseModule } from '@nestjs/mongoose';
import {commentSchema, courseSchema} from './course.model';
import {UserModule} from "../user/user.module";


@Module({

    imports: [

        MongooseModule.forFeature([{ name: 'Course', schema: courseSchema }, { name: 'Comment', schema: commentSchema }]),
        UserModule,
    ],
    controllers: [CourseController],
    providers: [CourseService],
})
export class CourseModule {}
