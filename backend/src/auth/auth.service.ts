// auth.service.ts
import * as fs from 'fs';
import * as path from 'path';
import { Injectable, UnauthorizedException } from '@nestjs/common';
import * as nodemailer from 'nodemailer';
import e from 'express';
import { UserModel } from 'src/user/user.model';
import { InjectModel } from '@nestjs/mongoose';
import { Model } from 'mongoose';
import { User } from 'src/user/user.model';
import { JwtAuthService } from 'src/jwt.service';
import { vertificationResponse, loginResponse, registerResponse, submitNicknameResponse, nickNameBodyDto } from './auth.dto';


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

  async register(user: { username: string; password: string; vertificationCode: string }): Promise<registerResponse> {
    const { username, password, vertificationCode } = user;

    // 检查email必须为z + 7位数字 + @ad.unsw.edu.au格式
    const emailRegex = /^z\d{7}@ad\.unsw\.edu\.au$/;
    if (!emailRegex.test(username)) {
      return new registerResponse(404, 'wrong email format');
    }

    // 密码要包含大小写字母加数字，长度大于等于8位
    const passwordRegex = /^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9]).{8,}$/;
    if (!passwordRegex.test(password)) {
      return new registerResponse(404, 'Password must contain upper and lower case letters and numbers, eight or more digits');
    }

    // 检查用户是否已经存在于MongoDB数据库中
    const existingUser = await this.userModel.findOne({ username: username });
    if (existingUser) {
      return new registerResponse(404, 'User already exists');
    }

    // 检查验证码是否正确
    if (vertificationCode !== '123456') {
      return new registerResponse(404, 'Verification code is incorrect');
    }

    //将用户添加到MongoDB数据库中
    const newUser = new this.userModel({ username, password });
    //给用户一个随机昵称按照格式user+7位随机数字,检查是否重复
    newUser.nickname = 'user' + Math.floor(Math.random() * 10000000);
    const existingNickname = await this.userModel.findOne({ nickname: newUser.nickname });
    if (existingNickname) {
      newUser.nickname = 'user' + Math.floor(Math.random() * 10000000);
    }

    await newUser.save();


    // 生成token
    const accessToken = await this.jwtService.generateAccessToken(username);
    const refreshToken = await this.jwtService.generateRefreshToken(username);

    newUser.refreshToken = refreshToken;
    await newUser.save();

    // 返回注册成功的信息
    return new registerResponse(200, 'Register successful', accessToken, refreshToken);
  }

  async sendVerificationCode(user: { username: string }): Promise<any> {
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
    return new vertificationResponse(200, 'Verification code sent');

  }

  async submitNickname(nickName: nickNameBodyDto, accessToken: string): Promise<submitNicknameResponse> {

    // 解析token
    const username = await this.jwtService.verifyToken(accessToken, 'access')

    // 检查用户是否存在于MongoDB数据库中
    const existingUser = await this.userModel.findOne({ username: username });
    if (!existingUser) {
      return new submitNicknameResponse(404, 'User does not exist');
    }

    // 检查昵称是否已经存在于MongoDB数据库中
    const existingNickname = await this.userModel.findOne({ nickname: nickName.nickName });
    if (existingNickname) {
      return new submitNicknameResponse(404, 'Nickname has been used');
    }

    // 更改此用户昵称
    existingUser.nickname = nickName.nickName;
    await existingUser.save();

    // 返回提交昵称成功的信息
    return new submitNicknameResponse(200, 'Submit nickname successful');
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
    
    const accessToken = await this.jwtService.generateAccessToken(username);
    const refreshToken = await this.jwtService.generateRefreshToken(username);

    existingUser.refreshToken = refreshToken;
    await existingUser.save();

    return new loginResponse(200, 'Login successful', accessToken, refreshToken);
  }

  async refreshAccessToken(refreshToken: string): Promise<any> {
    //判断当前refreshToken是否于数据库中的refreshToken一致
    const existingUser = await this.userModel.findOne({ refreshToken: refreshToken });
    if (!existingUser) {
      return new UnauthorizedException('Invalid token');
    }

    //verify看该refreshToken是否过期
    const username = await this.jwtService.verifyToken(refreshToken, 'refresh');

    // 生成新的token
    const accessToken = await this.jwtService.generateAccessToken(username);
    const newRefreshToken = await this.jwtService.generateRefreshToken(username);

    // 更新用户的refreshToken
    existingUser.refreshToken = newRefreshToken;
    await existingUser.save();

    // 返回新的token
    return new loginResponse(200, 'Refresh access token successful', accessToken, newRefreshToken);
  }
}