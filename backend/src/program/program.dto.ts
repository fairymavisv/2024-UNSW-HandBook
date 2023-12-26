import { ApiProperty } from '@nestjs/swagger';

export class CourseDto {
    @ApiProperty({ example: 'COMP3431', description: 'The code of the course' })
    public code: string;

    @ApiProperty({ example: 'Robotic Software Architecture', description: 'The name of the course' })
    public name: string;

    @ApiProperty({ example: 6, description: 'The Units of Credit (UOC) of the course' })
    public uoc: number;

    @ApiProperty({ example: 'An introduction to Intelligent agent design.', description: 'The description of the course' })
    public description: string;

    @ApiProperty({ example: ['COMP2521 or COMP1927', 'WAM of at least 70'], description: 'The list of prerequisites for the course' })
    public conditions: string[];

    @ApiProperty({ example: ['Term1', 'Term3'], description: 'The list of the offering term for the course' })
    public offerTerms: string[];

    // Constructor isn't necessary for DTOs as they are typically used for data transfer, not business logic
}

export class MajorDto {
    @ApiProperty({ example: 'Computer Science', description: 'The name of the major' })
    public name: string;

    @ApiProperty({ example: 48, description: 'Total Units of Credit (UOC) required for the major' })
    public UOC: number;

    @ApiProperty({ type: [CourseDto], description: 'List of compulsory courses for the major' })
    public CompulsoryCourseList: CourseDto[];

    @ApiProperty({ type: [CourseDto], description: 'List of specialized elective courses for the major' })
    public SpecializedElectiveCourses: CourseDto[];

    // Constructor isn't necessary for DTOs as they are typically used for data transfer, not business logic
}

export class ProgramDto {
    @ApiProperty({ example: 'Bachelor of Science', description: 'Name of the program' })
    name: string;

    @ApiProperty({ example: 48, description: 'Total Units of Credit (UOC) for the program' })
    UOC: number;

    @ApiProperty({ example: 'COMP6771', description: 'Code of the program' })
    code: string;

    @ApiProperty({ type: [MajorDto], description: 'List of majors available in the program' })
    majorList: MajorDto[];

    @ApiProperty({ type: [CourseDto], description: 'List of compulsory courses for the program' })
    CompulsoryCourseList: CourseDto[];

    @ApiProperty({ type: [CourseDto], description: 'List of specialized elective courses for the program' })
    SpecializedElectiveCourses: CourseDto[];

    // Constructor isn't necessary for DTOs as they are typically used for data transfer, not business logic
}
