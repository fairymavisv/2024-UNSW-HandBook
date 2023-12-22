// auth.service.ts
import * as fs from 'fs';
import * as path from 'path';
import { Injectable } from '@nestjs/common';

@Injectable()
export class AuthService {

  async register(user: { email: string; password: string; confirmPassword: string }): Promise<any> {
    const { email, password, confirmPassword } = user;

    // 检查密码和确认密码是否匹配
    if (password !== confirmPassword) {
      return { message: 'Password and Confirm Password do not match' };
    }

    // 检查用户是否已经存在于data目录下的users.json文件中
    const jsonPath = path.join(__dirname, '..', '..', 'data', 'users.json');
    const users = JSON.parse(fs.readFileSync(jsonPath, 'utf8'));

    if (users[email]) {
      return { message: 'User already exists' };
    }

    // 将用户添加到users.json文件中
    users[email] = { password };
    // 将用户初始昵称设置为邮箱前缀
    const nickname = email.split('@')[0];
    users[email].nickname = nickname;

    // 将更新后的用户信息写入users.json文件中
    fs.writeFileSync(jsonPath, JSON.stringify(users));

    // 返回注册成功的信息

    return { message: 'Registration successful' };
  }

  async authenticate(user: { email: string; vertificationCode: string; nickName: string }): Promise<any> {
      // 向用户发送验证码
      const { email, vertificationCode, nickName } = user;
  }

  async login(user: { email: string; password: string }): Promise<any> {
    // 检查用户是否存在于data目录下的users.json文件中
    const { email, password } = user;
    const jsonPath = path.join(__dirname, '..', '..', 'data', 'users.json');
    const users = JSON.parse(fs.readFileSync(jsonPath, 'utf8'));

    if (!users[email]) {
      return { message: 'User does not exist' };
    }

    // 检查密码是否正确
    if (users[email].password !== password) {
      return { message: 'Password is incorrect' };
    }

    // 返回登录成功的信息
    return { message: 'Login successful' };
  }
}