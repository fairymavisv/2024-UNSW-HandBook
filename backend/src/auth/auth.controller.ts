// auth.controller.ts

import { Body, Controller, Post } from '@nestjs/common';
import { AuthService } from './auth.service';

@Controller('auth')
export class AuthController {
  constructor(private readonly authService: AuthService) {}

  @Post('register')
  async register(@Body() body: { email: string; password: string; confirmPassword: string }): Promise<any> {
    return this.authService.register(body);
  }

  @Post('authenticate')
  async authenticate(@Body() body: { email: string; vertificationCode: string; nickName: string}): Promise<any> {
    return this.authService.authenticate(body);
  }

  @Post('login')
  async login(@Body() body: { email: string; password: string }): Promise<any> {
    return this.authService.login(body);
  }
}