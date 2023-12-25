// auth.service.ts
import * as fs from 'fs';
import * as path from 'path';
import { Injectable } from '@nestjs/common';
import * as nodemailer from 'nodemailer';
import e from 'express';

@Injectable()
export class AuthService {

  private transporter = nodemailer.createTransport({
    service: 'gmail',
    auth: {
      user: 'glenndeng@outlook.com', // 发件人邮箱
      pass: 'Dyc20020303.', // 发件人邮箱密码或应用程序密码
    },
  });

  async register(user: { email: string; password: string; confirmPassword: string }): Promise<any> {
    const { email, password, confirmPassword } = user;

    // 检查密码和确认密码是否匹配
    if (password !== confirmPassword) {
      return { message: 'Password and Confirm Password do not match' };
    }
    // 检查email必须为z + 7位数字 + @ad.unsw.edu.au格式
    const emailRegex = /^z\d{7}@ad\.unsw\.edu\.au$/;
    if (!emailRegex.test(email)) {
      return { message: 'Email format must be UNSW student email' };
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

  async sendVerificationCode(user: { email: string }): Promise<any> {
    // 像用户发送验证码
    /*
    const mailOptions = {
      from: 'glenndeng@outlook.com', // 发件人邮箱
      to: user.email, // 收件人邮箱，多个邮箱地址间用逗号分隔
      subject: 'UNSW HandbookX Email Verification', // 邮件标题
      text: '123456', // 邮件内容
    };

    return new Promise((resolve, reject) => {
      this.transporter.sendMail(mailOptions, (error, info) => {
        if (error) {
          console.log(error);
          reject({ message: 'Failed to send verification code' });
        } else {
          resolve({ message: 'Verification code sent' });
        }
      });
    });
    */
    return { message: 'Verification code sent' };

  }

  async submitNickname(user: { email: string; vertificationCode: string; nickName: string}): Promise<any> {
    // 检查用户是否存在于data目录下的users.json文件中
    const { email, vertificationCode, nickName } = user;
    const jsonPath = path.join(__dirname, '..', '..', 'data', 'users.json');
    const users = JSON.parse(fs.readFileSync(jsonPath, 'utf8'));

    if (!users[email]) {
      return { message: 'User does not exist' };
    }

    // 检查验证码是否正确
    if (vertificationCode !== '123456') {
      return { message: 'Verification code is incorrect' };
    }

    // 将用户昵称更新到users.json文件中
    users[email].nickname = nickName;
    fs.writeFileSync(jsonPath, JSON.stringify(users));

    // 返回提交昵称成功的信息
    return { message: 'Nickname submitted' };
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