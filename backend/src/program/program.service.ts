import { Injectable } from '@nestjs/common';
// import * as fs from 'fs';
// import * as path from 'path';
// import { Program, Course } from './program.model';
import { programInterface } from './program.interface';

@Injectable()
export class ProgramService {
    // private programs: { [key: string]: Program };

    constructor() {

    }

    async getProgramInfo(programCode: string): Promise<string[]> {
        const program = await programInterface.getProgramInfo(programCode);
        if (program) {
            return program.majorList && program.majorList.length > 0
                ? program.majorList.map(major => major.name)
                : program.CompulsoryCourseList;
        }else {
            throw new Error('Program not found');
        }
        return null;
    }

    async getCoursesForMajor(programCode: string, majorName: string,): Promise<string[]> {
        const major = await programInterface.getMajorInfo(programCode, majorName);
        if (major) {
            return major;
        }else {
            throw new Error('Major not found');
        }

        return null;
    }




}
