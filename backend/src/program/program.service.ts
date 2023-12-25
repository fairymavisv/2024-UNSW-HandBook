import { Injectable } from '@nestjs/common';
import * as fs from 'fs';
import * as path from 'path';
import { Program, Course } from './program.model';

@Injectable()
export class ProgramService {
    private programs: { [key: string]: Program };

    constructor() {
        const jsonPath = path.join(__dirname, '..', '..', 'data', 'programs.json');
        this.programs = JSON.parse(fs.readFileSync(jsonPath, 'utf8'));

    }

    getProgramInfo(programCode: string): string[] | Course[] {
        const program = this.programs[programCode];
        if (program) {
            return program.majorList && program.majorList.length > 0
                ? program.majorList.map(major => major.name)
                : program.CompulsoryCourseList;
        }else {
            throw new Error('Program not found');
        }
        return null;
    }

    getCoursesForMajor(programCode: string, majorName: string,): Course[] {
        const program = this.programs[programCode];
        if (program) {
            const major = program.majorList?.find(m => m.name === majorName);
            if (major) {
                return major.CompulsoryCourseList;
            }else {
                throw new Error('Major not found');
            }
        }else {
            throw new Error('Program not found');
        }

        return null;
    }




}
