import { Module } from '@nestjs/common';
import { UserController } from './user.controller';
import { UserService } from './user.service';
import { MongooseModule } from '@nestjs/mongoose';
import { userSchema } from './user.model';
import { JwtAuthService } from 'src/jwt.service';
import { JwtService } from '@nestjs/jwt';

@Module({
    imports: [

        MongooseModule.forFeature([{ name: 'User', schema: userSchema }]),

    ],
    exports: [UserService, JwtAuthService, JwtAuthService, MongooseModule.forFeature([{ name: 'User', schema: userSchema }])], // 导出 JwtAuthService
    controllers: [UserController],
    providers: [UserService,JwtAuthService, JwtService],
})
export class UserModule {}
