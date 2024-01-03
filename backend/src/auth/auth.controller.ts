// auth.controller.ts

import { Body, Controller, Post } from '@nestjs/common';
import { AuthService } from './auth.service';
import { loginBodyDto, loginResponse, nickNameBodyDto, registerBodyDto, registerResponse, sendVerificationCodeDto, submitNicknameResponse, vertificationResponse } from './auth.dto';
import { ApiResponse } from '@nestjs/swagger';

@Controller('auth')
export class AuthController {
  constructor(private readonly authService: AuthService) {}

  @Post('register')
  @ApiResponse({ status: 500, description: 'internal error' })
  @ApiResponse({
    status: 200,
    description: 'register success',
    type: registerResponse
  })
  async register(@Body() body: registerBodyDto): Promise<any> {
    return this.authService.register(body);
  }

  @Post('send-verification-code')
  @ApiResponse({ status: 500, description: 'internal error' })
  @ApiResponse({
    status: 200,
    description: 'register success',
    type: vertificationResponse
  })
  async sendVerificationCode(@Body() body: sendVerificationCodeDto): Promise<any> {

    return await this.authService.sendVerificationCode(body);
  }

  @Post('submitNickname')
  @ApiResponse({ status: 404, description: 'user not found' })
  @ApiResponse({
      status: 200,
      description: 'submit nickname success',
      type: submitNicknameResponse
  })
  async submitNickname(@Body() body: nickNameBodyDto): Promise<any> {
    return this.authService.submitNickname(body);
  }

  @Post('login')
  @ApiResponse({ status: 404, description: 'user not found' })
  @ApiResponse({
      status: 200,
      description: 'login success',
      type: loginResponse
  })
  async login(@Body() body: loginBodyDto): Promise<any> {
    return this.authService.login(body);
  }
}