import { Injectable } from '@nestjs/common';
import * as fs from 'fs';
import * as path from 'path';
import { Program, Course } from './program.model';

@Injectable()
export class ProgramService {
    private programs: { [key: string]: Program };
    private recentProgram: Program | null = null;

    constructor() {
        const jsonPath = path.join(__dirname, '..', '..', 'data', 'programs.json');
        this.programs = JSON.parse(fs.readFileSync(jsonPath, 'utf8'));

    }

    getProgramInfo(programCode: string): string[] | Course[] {
        const program = this.programs[programCode];
        if (program) {
            this.recentProgram = program; // 缓存当前 program
            return program.majorList && program.majorList.length > 0
                ? program.majorList.map(major => major.name)
                : program.CompulsoryCourseList;
        }else {

        }
        return null; // 或者抛出一个错误，如果 program 不存在
    }

    getCoursesForMajor(majorName: string): Course[] {
        if (this.recentProgram) {
            const major = this.recentProgram.majorList?.find(m => m.name === majorName);
            if (major) {
                return major.CompulsoryCourseList;
            }
        }
        return null; // 或者抛出一个错误，如果 major 不存在
    }




}
