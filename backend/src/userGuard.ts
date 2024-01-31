import { CanActivate, ExecutionContext, Injectable } from '@nestjs/common';
import { JwtAuthService } from './jwt.service';
import { JwtService } from '@nestjs/jwt';
import { InjectModel } from '@nestjs/mongoose';
import { Model } from 'mongoose';
import { User } from 'src/user/user.model';

@Injectable()
export class UserGuard implements CanActivate {
  constructor(@InjectModel('User') private userModel: Model<User>, private jwtService: JwtAuthService) {}

  canActivate(context: ExecutionContext): boolean | Promise<boolean>{
    const request = context.switchToHttp().getRequest();
    const authorization = request.headers.authorization;
    const accessToken = authorization.replace('Bearer ', '');
    const username = this.jwtService.verifyToken(accessToken, 'access')
    // 检查用户是否存在于MongoDB数据库中
    const existingUser = this.userModel.findOne({ username: username });
    if (!existingUser) {
      return false;
    }
    return username
  }
}