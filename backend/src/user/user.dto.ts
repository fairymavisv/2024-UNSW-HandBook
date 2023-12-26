import { ApiProperty } from '@nestjs/swagger';

export class UserDto {
    @ApiProperty({ example: 'johndoe', description: 'The username of the user' })
    username: string;

    @ApiProperty({ example: ['program1'], description: 'user programs' })
    programs: string;

    @ApiProperty({ example: ['course1', 'course2'], description: 'List of courses' })
    courseslist: string[];

    // ...其他字段，排除敏感信息如密码
}
