import { ApiProperty } from '@nestjs/swagger';

export class registerBodyDto {
    @ApiProperty({ example: 'z5362100@ad.unsw.edu.au', description: 'The UNSW Email of the user' })
    username: string;

    @ApiProperty({ example: 'Dyc20020303', description: 'The password contains upper and lower case letters and numbers, eight or more digits.' })
    password: string;

    @ApiProperty({ example: 'Dyc20020303', description: 'The password contains upper and lower case letters and numbers, eight or more digits.' })
    confirmPassword: string;

}

export class sendVerificationCodeDto {
    @ApiProperty()
    email: string;
}

export class nickNameBodyDto {

    @ApiProperty({ example: {
        username: "z5362103@ad.unsw.edu.au",
        password: "Dyc21120303"
    }, description: 'The UNSW Email and password of the user' })
    userData: { username: string; password: string };

    @ApiProperty({ example: '123456', description: 'The vertification code' })
    vertificationCode: string;

    @ApiProperty({ example: 'Glenn', description: 'The nickname of the user' })
    nickName: string;

}

export class loginBodyDto {
    @ApiProperty({ example: 'z5362100@ad.unsw.edu.au', description: 'The UNSW Email of the user' })
    username: string;

    @ApiProperty({ example: 'Dyc20020303', description: 'The password contains upper and lower case letters and numbers, eight or more digits.' })
    password: string;
}

export class loginResponse {
    @ApiProperty({ example: 200, description: 'The status code of the response' })
    statusCode: number;
  
    @ApiProperty()
    message: string;
  
    @ApiProperty()
    token?: string;
  
    constructor(statusCode: number, message: string, token?: string) {
      this.statusCode = statusCode;
      this.message = message;
      this.token = token;
    }
}

export class registerResponse {
    @ApiProperty({ example: 200, description: 'The status code of the response' })
    statusCode: number;
  
    @ApiProperty()
    message: string;

    @ApiProperty({ example: {
        username: "z5362103@ad.unsw.edu.au",
        password: "Dyc21120303"
    }, description: 'The UNSW Email and password of the user' })
    userData?: { username: string; password: string };

    constructor(statusCode: number, message: string) {
        this.statusCode = statusCode;
        this.message = message;
        this.userData = { username: '', password: '' };
    }

}

export class authResponse {
    @ApiProperty({ example: 200, description: 'The status code of the response' })
    statusCode: number;
  
    @ApiProperty()
    message: string;

    constructor(statusCode: number, message: string) {
        this.statusCode = statusCode;
        this.message = message;
    }

}