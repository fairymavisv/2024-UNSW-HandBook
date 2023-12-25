// auth.controller.ts

import { Body, Controller, Post } from '@nestjs/common';
import { AuthService } from './auth.service';

@Controller('auth')
export class AuthController {
  constructor(private readonly authService: AuthService) {}

  @Post('register')
  async register(@Body() body: { username: string; password: string; confirmPassword: string }): Promise<any> {
    return this.authService.register(body);
  }

  @Post('send-verification-code')
  async sendVerificationCode(@Body() body: { email: string }): Promise<any> {

    return await this.authService.sendVerificationCode(body);
  }

  @Post('submitNickname')
  async submitNickname(@Body() body: { username: string; vertificationCode: string; nickName: string}): Promise<any> {
    return this.authService.submitNickname(body);
  }

  @Post('login')
  async login(@Body() body: { username: string; password: string }): Promise<any> {
    return this.authService.login(body);
  }
}