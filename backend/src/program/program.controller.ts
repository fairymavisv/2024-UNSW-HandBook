import {Controller, Get, HttpException, HttpStatus, Param, Query} from '@nestjs/common';
import { ProgramService } from './program.service';
import {ApiOperation, ApiResponse} from "@nestjs/swagger";
import {MajorDto, ProgramDto} from "./program.dto";

@Controller('programs')
export class ProgramController {
    constructor(private readonly programService: ProgramService) {}


    @Get(':programCode')
    @ApiOperation({ summary: 'Get given program info' })
    @ApiResponse({ status: 404, description: 'program code not found' })
    @ApiResponse({
        status: 200,
        description: 'The user details',
        type: ProgramDto, // 指定返回的类型是 UserDto
    })
    async getProgramInfo(@Param('programCode') programCode: string) {
        try {
            const programInfo = await this.programService.getProgramInfo(programCode);
            if (!programInfo) {
                throw new HttpException('Program not found', HttpStatus.NOT_FOUND);
            }
            return programInfo;
        } catch (error) {
            throw new HttpException('Internal server error', HttpStatus.INTERNAL_SERVER_ERROR);
        }
    }

    @Get(':programCode/:majorName')
    @ApiOperation({summary: 'Get given program info'})
    @ApiResponse({status: 404, description: 'program code not found'})
    @ApiResponse({
        status: 200,
        description: 'The user details',
        type: MajorDto, // 指定返回的类型是 UserDto
    })
    async getCoursesForMajor(@Param('programCode') programCode: string,
                             @Param('majorName') majorName: string) {
        try {
            const decodedMajorName = decodeURIComponent(majorName);
            const courses = this.programService.getCoursesForMajor(programCode, majorName);
            return courses;
        } catch (error) {
            throw new HttpException('Failed to fetch courses for major', HttpStatus.BAD_REQUEST);
        }
    }
}
