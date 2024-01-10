import { Module } from '@nestjs/common';
import { AuthController } from './auth.controller';
import { AuthService } from './auth.service';
import { MongooseModule } from '@nestjs/mongoose';
import { userSchema } from '../user/user.model';
import { ConfigModule } from '@nestjs/config';
import { JwtAuthService } from 'src/jwt.service';
import { JwtService } from '@nestjs/jwt';

@Module({
    imports: [
        ConfigModule.forRoot({
            isGlobal: true,
        }),
        MongooseModule.forFeature([{ name: 'User', schema: userSchema }]),
        MongooseModule.forRoot(process.env.MONGO_URI)
    ],
    controllers: [AuthController],
    providers: [AuthService, JwtAuthService, JwtService],
})
export class AuthModule {}

