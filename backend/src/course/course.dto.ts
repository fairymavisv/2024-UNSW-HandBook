import { ApiProperty } from '@nestjs/swagger';

export class CommentDto {

    @ApiProperty({ example: 'This course is great!', description: 'The text of the comment' })
    public text: string;

    @ApiProperty({ example: 5, description: 'The rating given by the user' })
    public rating: number;

    @ApiProperty({ example: '2021-04-01T00:00:00.000Z', description: 'The date and time the comment was last updated' })
    public updatedAt: Date;

    @ApiProperty({ example: 'tron', description: 'The username of the user who made the comment' })
    public username?: string;
}


export class CreateCommentDto {
    //顺序跟前端传过来的顺序一致
    @ApiProperty({ example: 'COMP3431', description: 'The code of the course' })
    courseCode: string;

    @ApiProperty({ example: '1234567890abcdef', description: 'The ID of the user' })
    userId: string;

    @ApiProperty({ example: 'This course is great!', description: 'The text of the comment' })
    text: string;

    @ApiProperty({ example: 5, description: 'The rating given by the user' })
    rating: number;
}


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

    @ApiProperty({ type: [CommentDto], description: 'comments of the course' })
    public comments : CommentDto[];

    // Constructor isn't necessary for DTOs as they are typically used for data transfer, not business logic
}

