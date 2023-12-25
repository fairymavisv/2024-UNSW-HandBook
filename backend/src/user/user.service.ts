import {Injectable, NotFoundException} from '@nestjs/common';
import { InjectModel } from '@nestjs/mongoose';
import { Model } from 'mongoose';
import {User} from './user.model';

@Injectable()
export class UserService {
    constructor(@InjectModel('User') private userModel: Model<User>) {}

    async addUserCourses(username: string, courseIds: string[]): Promise<User> {
        const user = await this.userModel.findOne({username: username});
        if (user) {
            console.log('find user')
            // 确保 courseslist 字段存在
            if (!user.courseslist) {
                user.courseslist = [];
            }
            console.log('find user courseslist')

            user.courseslist = Array.from(new Set([...user.courseslist, ...courseIds]));
            console.log('user courselist', user.courseslist)
            await user.save();
        }
        return user;
    }

    async updateUserCourse(username: string, courseId: string[]): Promise<User> {
        const user = await this.userModel.findOne({username: username});
        if (user) {
            console.log('find user')
            // 确保 courseslist 字段存在
            if (!user.courseslist) {
                user.courseslist = [];
            }
            console.log('find user courseslist')

            user.courseslist = user.courseslist.filter(m => !courseId.includes(m));
            console.log('updated user courseslist', user.courseslist);
            await user.save();
        }
        return user;
    }


    async getUser(username: string): Promise<User> {
        const user = await this.userModel.findOne({username: username});


        console.log('Fetching user with username:', username);
        console.log('Found user:', user);
        if (!user) {
            throw new NotFoundException(`User with username ${username} not found`);
        }
        return user;

    }

    async getUserCourses(username: string) {
        const user = await this.userModel.findOne({username: username});
        if (user) {
            return user.courseslist;
        }else {
            throw new NotFoundException(`User with username ${username} not found`);
        }

    }
}

