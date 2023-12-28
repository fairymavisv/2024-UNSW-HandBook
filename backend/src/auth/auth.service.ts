// auth.service.ts
import * as fs from 'fs';
import * as path from 'path';
import { Injectable } from '@nestjs/common';
import * as nodemailer from 'nodemailer';
import e from 'express';
import { userModel } from 'src/user/user.model';
import { InjectModel } from '@nestjs/mongoose';
import { Model } from 'mongoose';
import { User } from 'src/user/user.model';
import { JwtAuthService } from 'src/jwt.service';
import { authResponse, loginResponse } from './auth.dto';


@Injectable()
export class AuthService {

  constructor(@InjectModel('User') private userModel: Model<User>, private jwtService: JwtAuthService) {}
  /*
  private transporter = nodemailer.createTransport({
    service: 'gmail',
    auth: {
      user: 'glenndeng@outlook.com', // 发件人邮箱
      pass: 'Dyc20020303.', // 发件人邮箱密码或应用程序密码
    },
  });
  */

  async register(user: { username: string; password: string; confirmPassword: string }): Promise<loginResponse> {
    const { username, password, confirmPassword } = user;

    // 检查密码和确认密码是否匹配
    if (password !== confirmPassword) {
      return new loginResponse(404, 'Password and confirm password do not match');
    }
    // 检查email必须为z + 7位数字 + @ad.unsw.edu.au格式
    const emailRegex = /^z\d{7}@ad\.unsw\.edu\.au$/;
    if (!emailRegex.test(username)) {
      return new loginResponse(404, 'wrong email format');
    }

    // 密码要包含大小写字母加数字，长度大于等于8位
    const passwordRegex = /^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9]).{8,}$/;
    if (!passwordRegex.test(password)) {
      return new loginResponse(404, 'Password must contain at least 8 characters, including uppercase, lowercase and numbers');
    }

    // 检查用户是否已经存在于MongoDB数据库中
    const existingUser = await this.userModel.findOne({ username: username });
    if (existingUser) {
      return new loginResponse(404, 'User already exists');
    }

    // 将用户添加到MongoDB数据库中
    const newUser = new this.userModel({ username, password });
    await newUser.save();

    // 生成token
    const token = await this.jwtService.generateToken(username);
    return { statusCode: 200, message: 'Register successful', token };
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
    return new authResponse(200, 'Verification code sent');

  }

  async submitNickname(user: { token: string; vertificationCode: string; nickName: string}): Promise<authResponse> {
    // 检查用户是否存在于MongoDB数据库中
    const { token, vertificationCode, nickName } = user;
    // 解析token
    const username = await this.jwtService.verifyToken(token);
    console.log(username);
    const existingUser = await this.userModel.findOne({ username: username });
    if (!existingUser) {
      return new authResponse(404, 'User does not exist');
    }

    // 检查验证码是否正确
    if (vertificationCode !== '123456') {
      return new authResponse(404, 'Verification code is incorrect');
    }

    // 将用户昵称更新到MongoDB数据库中
    existingUser.nickname = nickName;
    await existingUser.save();

    // 返回提交昵称成功的信息
    return new authResponse(200, 'Submit nickname successful');
  }


  async login(user: { username: string; password: string }): Promise<loginResponse> {
    const { username, password } = user;
    const existingUser = await this.userModel.findOne({ username: username });
    if (!existingUser) {
      return new loginResponse(404, 'User does not exist');
    }

    // 检查密码是否正确
    if (password !== existingUser.password) {
      return new loginResponse(404, 'Password is incorrect');
    }
    // console.log(existingUser)
    const token = await this.jwtService.generateToken(username);
    new loginResponse(200, 'Login successful', token);

    return new loginResponse(200, 'Login successful', token);
  }
}