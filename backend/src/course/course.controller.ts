import {Controller, Get, Post, HttpException, HttpStatus, Param, Put, Query, Body, Delete} from '@nestjs/common';
import {CourseService } from './course.service';
import {ApiOperation, ApiResponse} from "@nestjs/swagger";
import {DeleteCommentDto,CourseInfoDto, CreateCommentReturnDto,CreateCommentDto} from "./course.dto";




@Controller('course')
export class CourseController {
    constructor(private readonly programService: CourseService) {}


    @Get(':CourseCode')
    @ApiOperation({ summary: 'Get given Course info' })
    @ApiResponse({ status: 404, description: 'Course code not found' })
    @ApiResponse({
        status: 200,
        description: 'The Course details',
        type: CourseInfoDto,
    })
    async getCourseInfo(@Param('CourseCode') CourseCode: string) {
        try {
            //统一将输入的 code 转化为大写
            const upperCaseCode = CourseCode.toUpperCase();
            const CourseInfo = await this.programService.getCourseInfo(upperCaseCode);
            if (!CourseInfo) {
                throw new HttpException('course not found', HttpStatus.NOT_FOUND);
            }
            return CourseInfo;
        } catch (error) {
            throw new HttpException(error.message, HttpStatus.INTERNAL_SERVER_ERROR);
        }
    }

    @Post("comment")
    @ApiOperation({ summary: 'create Course comment' })
    @ApiResponse({ status: 404, description: 'Course code not found' })
    @ApiResponse({
        status: 200,
        description: 'The Course details',
        type: CreateCommentReturnDto, // 指定返回的类型是 UserDto
    })
    async createCourseComment(@Body() createCommentDto: CreateCommentDto) {
        try {
            const comment = await this.programService.createCourseComment(createCommentDto);
            return comment;
        } catch (error) {
            // Handle exceptions thrown by the service
            throw new HttpException(error.message, error.status || HttpStatus.INTERNAL_SERVER_ERROR);
        }
    }

    @Delete("comment")
    @ApiOperation({ summary: 'delete Course comment' })
    @ApiResponse({ status: 404, description: 'Course code not found' })
    @ApiResponse({
        status: 200,
        description: 'The Course details',
        type: String,
    })
    async deleteCourseComment(@Body() deleteCommentDto: DeleteCommentDto){
        try {

            const ret = await this.programService.deleteCourseComment(deleteCommentDto);
            if (!ret) {
                throw new HttpException('delete comment failed', HttpStatus.NOT_FOUND);
            }
            return ret;
        } catch (error) {
            throw new HttpException(error.message, HttpStatus.INTERNAL_SERVER_ERROR);
        }
    }


}
