import {Controller, Get, HttpException, HttpStatus, Param, Query} from '@nestjs/common';
import { CourseService } from './course.service';
import {ApiOperation, ApiResponse} from "@nestjs/swagger";
import {CourseDto} from "./course.dto";

@Controller('course')
export class CourseController {
    constructor(private readonly programService: CourseService) {}


    @Get(':CourseCode')
    @ApiOperation({ summary: 'Get given Course info' })
    @ApiResponse({ status: 404, description: 'Course code not found' })
    @ApiResponse({
        status: 200,
        description: 'The Course details',
        type: CourseDto, // 指定返回的类型是 UserDto
    })
    async getCourseInfo(@Param('CourseCode') CourseCode: string) {
        try {
            //统一将输入的 code 转化为大写
            const upperCaseCode = CourseCode.toUpperCase();
            const CourseInfo = await this.programService.getCourseInfo(upperCaseCode);
            if (!CourseInfo) {
                throw new HttpException('Program not found', HttpStatus.NOT_FOUND);
            }
            return CourseInfo;
        } catch (error) {
            throw new HttpException('Internal server error', HttpStatus.INTERNAL_SERVER_ERROR);
        }
    }

    // @Get(':programCode/:majorName')
    // @ApiOperation({summary: 'Get given program info'})
    // @ApiResponse({status: 404, description: 'program code not found'})
    // @ApiResponse({
    //     status: 200,
    //     description: 'The user details',
    //     type: MajorDto, // 指定返回的类型是 UserDto
    // })
    // async getCoursesForMajor(@Param('programCode') programCode: string,
    //                          @Param('majorName') majorName: string) {
    //     try {
    //         const decodedMajorName = decodeURIComponent(majorName);
    //         const courses = this.programService.getCoursesForMajor(programCode, majorName);
    //         return courses;
    //     } catch (error) {
    //         throw new HttpException('Failed to fetch courses for major', HttpStatus.BAD_REQUEST);
    //     }
    // }
}
