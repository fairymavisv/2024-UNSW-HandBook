import { ApiProperty } from '@nestjs/swagger';


export class BaseCommentDto {
    @ApiProperty({ example: 'This course is great!', description: 'The text of the comment' })
    public text: string;

    @ApiProperty({ example: '4', description: 'The difficulty of the course' })
    public difficulty: number;

    @ApiProperty({ example: '5', description: 'The usefulness of the course' })
    public usefulness: number;

    @ApiProperty({ example: '3', description: 'The workload of the course' })
    public workload: number;
}

export class CommentDto extends BaseCommentDto {
    @ApiProperty({ example: '2021-04-01T00:00:00.000Z', description: 'The date and time the comment was last updated' })
    public updatedAt: Date;

    @ApiProperty({ example: 'tron', description: 'The username of the user who made the comment' })
    public username?: string;
}

export class CreateCommentDto extends BaseCommentDto {
    @ApiProperty({ example: 'COMP3431', description: 'The code of the course' })
    courseCode: string;

    @ApiProperty({ example: 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Ino1NTU1NTU1QGFkLnVuc3cuZWR1LmF1IiwiaWF0IjoxNzA1MDI0ODM2LCJleHAiOjE3MDUwMjU3MzZ9.T802GRXVQIyGZpBj8i3ybfuBx3A8iC1kCunKn0p9i0s', description: 'The token of the user' })
    token: string;
}

export class CreateCommentReturnDto extends CommentDto {
    @ApiProperty({ example: '65a098701bb86821f6e95be8', description: 'The id of the comment' })
    _id: string;
}

export class DeleteCommentDto {
    @ApiProperty({ example: '65a098701bb86821f6e95be8', description: 'The id of the comment' })
    commentID: string;

    @ApiProperty({ example: 'token', description: 'The token of the user' })
    token: string;
}

export class BasicCourseInfoDto {
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
    public offerterms: string[];
}

export class CourseInfoDto{
    @ApiProperty({ type: BasicCourseInfoDto, description: 'The basic information of the course' })
    basicInfo: BasicCourseInfoDto;
    @ApiProperty({ type: [CommentDto], description: 'The comments of the course' })
    comments: CommentDto[];
}