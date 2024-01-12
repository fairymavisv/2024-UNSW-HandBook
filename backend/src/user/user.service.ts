import {Injectable, NotFoundException} from '@nestjs/common';
import { InjectModel } from '@nestjs/mongoose';
import { Model } from 'mongoose';
import {User} from './user.model';
import {createProfileDto, } from "./user.dto";
import {CourseInterface} from "../course/course.interface";
import { JwtAuthService } from 'src/jwt.service';

@Injectable()
export class UserService {
    constructor(@InjectModel('User') private userModel: Model<User>, private jwtService: JwtAuthService) {}


    // update user courses list
    async updateUserCourse(token: string, courseId: string[]) {
        const username = await this.jwtService.verifyToken(token, 'access');
        const user = await this.userModel.findOne({ username: username });

        if (user) {
            console.log('find user');

            // 确保 courseslist 字段存在
            if (!user.courseslist) {
                user.courseslist = [];
            }
            console.log('find user courseslist');

            // 将 courseId 中的所有课程 ID 转换为大写
            user.courseslist = courseId.map(id => id.toUpperCase());
            console.log('updated user courseslist', user.courseslist);

            await user.save();
        } else {
            throw new NotFoundException(`User with username ${username} not found or Authentication failed`);
        }
        return user.courseslist;
    }


    async getUser(token: string): Promise<User> {
        const username = await this.jwtService.verifyToken(token, 'access');
        console.log('token:', token);
        console.log('username:', username);
        const user = await this.userModel.findOne({username: username}).select('-_id');;


        console.log('Found user:', user);
        if (!user) {
            throw new NotFoundException(`User with username ${username} not found or Authentication failed`);
        }
        return user;

    }

    async getUserCourses(token: string) {
        // 解析token
        const username = await this.jwtService.verifyToken(token, 'access');
        console.log('get_user_course username:', username);
        const user = await this.userModel.findOne({username: username});
        if (user) {
            return user.courseslist;
        }else {
            throw new NotFoundException(`User with username ${username} not found or Authentication failed`);
        }

    }

    //create or update user profile

    async createUserProfile(user:createProfileDto) {
        // 解析token
        const username = await this.jwtService.verifyToken(user.token, 'access');
        console.log('create_profiles_username:', username);
        // 检查用户是否存在于MongoDB数据库中
        let userfromMoogo = await this.userModel.findOne({username: username});

        if(!userfromMoogo){
            throw new NotFoundException(`User with username ${username} not found or Authentication failed`);
        }
        console.log('before', userfromMoogo);
        userfromMoogo.program = user.program;
        userfromMoogo.major = user.major;
        await userfromMoogo.save();
        console.log('after', userfromMoogo);
        return username + '\'s profile has been upgraded';
    }


}

