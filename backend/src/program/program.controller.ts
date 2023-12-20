import { Controller, Get, Param } from '@nestjs/common';
import { ProgramService } from './program.service';

@Controller('programs')
export class ProgramController {
    constructor(private readonly programService: ProgramService) {}

    @Get(':programCode')
    getProgramInfo(@Param('programCode') programCode: string) {
        return this.programService.getProgramInfo(programCode);
    }

    @Get(':programCode/:majorName')
    getCoursesForMajor(@Param('majorName') majorName: string) {
        const decodedMajorName = decodeURIComponent(majorName);
        return this.programService.getCoursesForMajor(decodedMajorName);
    }
}
