import {Body, Controller, Get, HttpException, HttpStatus, Param, Post, Put} from '@nestjs/common';
import { UserService } from './user.service';
import {User} from "./user.model";
import {ApiResponse} from "@nestjs/swagger";
import {UserDto} from "./user.dto";

@Controller('users')
export class UserController {
    constructor(private readonly userService: UserService) {}

    @Get(':username')
    @ApiResponse({ status: 404, description: 'user not found' })
    @ApiResponse({
        status: 200,
        description: 'The user details',
        type: UserDto, // 指定返回的类型是 UserDto
    })
    async getUser(@Param('username') username: string): Promise<User> {
        try {
            const user = await this.userService.getUser(username);
            if (!user) {
                throw new HttpException('User not found', HttpStatus.NOT_FOUND);
            }
            return user;
        } catch (error) {
            throw new HttpException('Internal server error', HttpStatus.INTERNAL_SERVER_ERROR);
        }
    }

    // get user courses list
    @Get(':username/courseslist')
    @ApiResponse({ status: 404, description: 'user not found' })
    @ApiResponse({
        status: 200,
        description: 'The user details',
        type: String,
        isArray: true // 明确指出返回的是字符串数组
    })
    async getUserCourses(@Param('username') username: string, @Body() courseIds: string[]){
        try {
            const updatedUser = await this.userService.getUserCourses(username);
            return updatedUser;
        } catch (error) {
            throw new HttpException('Failed to get user courses', HttpStatus.BAD_REQUEST);
        }
    }

    // add user courses list
    @Post(':username/courseslist')
    @ApiResponse({ status: 404, description: 'user not found' })
    @ApiResponse({
        status: 200,
        description: 'The user details',
        type: String,
        isArray: true
    })
    async addUserCourses(@Param('username') username: string, @Body() courseIds: string[]): Promise<User> {
        try {
            const updatedUser = await this.userService.addUserCourses(username, courseIds);
            return updatedUser;
        } catch (error) {
            throw new HttpException('Failed to add courses', HttpStatus.BAD_REQUEST);
        }
    }
    // update user courses list
    @Put(':username/courseslist')
    @ApiResponse({ status: 404, description: 'user not found' })
    @ApiResponse({
        status: 200,
        description: 'The user details',
        type: String,
        isArray: true
    })
    async updateUserCourse(@Param('username') username: string, @Body() courseIds: string[]): Promise<User> {
        try {
            const updatedUser = await this.userService.updateUserCourse(username, courseIds);
            return updatedUser;
        } catch (error) {
            throw new HttpException('Failed to update course status', HttpStatus.BAD_REQUEST);
        }
    }
}
