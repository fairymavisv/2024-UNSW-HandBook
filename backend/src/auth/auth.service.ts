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


@Injectable()
export class AuthService {

  constructor(@InjectModel('User') private userModel: Model<User>) {}
  /*
  private transporter = nodemailer.createTransport({
    service: 'gmail',
    auth: {
      user: 'glenndeng@outlook.com', // 发件人邮箱
      pass: 'Dyc20020303.', // 发件人邮箱密码或应用程序密码
    },
  });
  */

  async register(user: { username: string; password: string; confirmPassword: string }): Promise<any> {
    const { username, password, confirmPassword } = user;

    // 检查密码和确认密码是否匹配
    if (password !== confirmPassword) {
      return { message: 'Password and Confirm Password do not match' };
    }
    // 检查email必须为z + 7位数字 + @ad.unsw.edu.au格式
    const emailRegex = /^z\d{7}@ad\.unsw\.edu\.au$/;
    if (!emailRegex.test(username)) {
      return { message: 'Email format must be UNSW student email' };
    }

    // 密码要包含大小写字母加数字，长度大于等于8位
    const passwordRegex = /^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9]).{8,}$/;
    if (!passwordRegex.test(password)) {
      return { message: 'Password must contain at least one uppercase letter, one lowercase letter and one number' };
    }

    // 检查用户是否已经存在于MongoDB数据库中
    const existingUser = await this.userModel.findOne({ username: username });
    if (existingUser) {
      return { message: 'User already exists' };
    }

    console.log('##################');
    // 将用户添加到MongoDB数据库中
    const newUser = new this.userModel({ username, password });
    await newUser.save();

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

  async submitNickname(user: { username: string; vertificationCode: string; nickName: string}): Promise<any> {
    // 检查用户是否存在于MongoDB数据库中
    const { username, vertificationCode, nickName } = user;
    const existingUser = await this.userModel.findOne({ username: username });
    if (!existingUser) {
      return { message: 'User does not exist' };
    }

    // 检查验证码是否正确
    if (vertificationCode !== '123456') {
      return { message: 'Verification code is incorrect' };
    }

    // 将用户昵称更新到MongoDB数据库中
    existingUser.nickname = nickName;
    await existingUser.save();

    // 返回提交昵称成功的信息
    return { message: 'Nickname submitted' };
  }


  async login(user: { username: string; password: string }): Promise<any> {
    const { username, password } = user;
    const existingUser = await this.userModel.findOne({ username: username });
    if (!existingUser) {
      return { message: 'User does not exist' };
    }

    // 检查密码是否正确
    if (password !== existingUser.password) {
      return { message: 'Password is incorrect' };
    }

    return { message: 'Login successful' };
  }
}