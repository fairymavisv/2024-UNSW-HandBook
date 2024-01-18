import {Body, Controller, Get, HttpException, HttpStatus, Param, Post, Put, Query} from '@nestjs/common';
import { UserService } from './user.service';
import {User} from "./user.model";
import {ApiResponse} from "@nestjs/swagger";
import {createProfileDto, UserDto} from "./user.dto";


@Controller('users')
export class UserController {
    constructor(private readonly userService: UserService) {}

    @Get()
    @ApiResponse({ status: 404, description: 'user not found' })
    @ApiResponse({
        status: 200,
        description: 'The user details',
        type: UserDto, // 指定返回的类型是 UserDto
    })
    async getUser(@Query('token') token: string) {
        try {
            const user = await this.userService.getUser(token);
            return user;
        } catch (error) {//TODO: error message可能会泄露内部实现,是否要该为更加通用的提示
            throw new HttpException(error.message, HttpStatus.INTERNAL_SERVER_ERROR);
        }
    }

    @Post('createProfile')
    @ApiResponse({ status: 500, description: 'internal error' })
    @ApiResponse({
        status: 200,
        description: 'create user profile success',
        type: String, // 返回成功信息
    })
    async createUserProfile(@Body() user:createProfileDto): Promise<string> {
        try {
            const ret = await this.userService.createUserProfile(user);
            return ret;
        } catch (error) {
            throw new HttpException(error.message, HttpStatus.BAD_REQUEST);
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
    async getUserCourses(@Param('token') token: string, @Body() courseIds: string[]){
        try {
            const updatedUser = await this.userService.getUserCourses(token);
            return updatedUser;
        } catch (error) {
            throw new HttpException(error.message, HttpStatus.BAD_REQUEST);
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
    async addUserCourses(@Param('token') token: string, @Body() courseIds: string[]): Promise<User> {
        try {
            const updatedUser = await this.userService.addUserCourses(token, courseIds);
            return updatedUser;
        } catch (error) {
            throw new HttpException(error.message, HttpStatus.BAD_REQUEST);
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
    async updateUserCourse(@Param('token') token: string, @Body() courseIds: string[]): Promise<User> {
        try {
            const updatedUser = await this.userService.updateUserCourse(token, courseIds);
            return updatedUser;
        } catch (error) {
            throw new HttpException(error.message, HttpStatus.BAD_REQUEST);
        }
    }



}
