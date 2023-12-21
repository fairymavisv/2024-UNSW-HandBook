import {Body, Controller, Get, Param, Post, Put} from '@nestjs/common';
import { UserService } from './user.service';
import {User} from "./user.model";

@Controller('users')
export class UserController {
    constructor(private readonly userService: UserService) {}

    @Get(':username')
    async getUser(@Param('username') username: string): Promise<User> {
        return this.userService.getUser(username);
    }


    @Post(':username/courseslist')
    async addCourses(@Param('username') username: string, @Body() courseIds: string[]): Promise<User> {
        return this.userService.addCourses(username, courseIds);
    }

    @Put(':username/courseslist')
    async updateCourseStatus(@Param('username') username: string, @Body() courseIds: string[]): Promise<User> {
        return this.userService.updateCourseStatus(username, courseIds);
    }
}
