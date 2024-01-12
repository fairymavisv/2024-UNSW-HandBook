import { ApiProperty } from '@nestjs/swagger';

export class createProfileDto {
    @ApiProperty({ example: 'sdfhskldhf82hsfdhjkf', description: 'The username of the user' })
    token: string;

    @ApiProperty({ example: 'program1', description: 'user programs' })
    program: string;

    @ApiProperty({ example: 'major1', description: 'user major' })
    major?: string;

}


export class UserDto {
    @ApiProperty({ example: 'johndoe', description: 'The username of the user' })
    username: string;

    @ApiProperty({ example: ['program1'], description: 'user programs' })
    program: string;

    @ApiProperty({ example: ['course1', 'course2'], description: 'List of courses' })
    courseslist: string[];

    // ...其他字段，排除敏感信息如密码
}
