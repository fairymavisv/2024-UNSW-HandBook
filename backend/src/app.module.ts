import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { ProgramModule } from './program/program.module';
import { MongooseModule } from '@nestjs/mongoose';
import {UserModule} from "./user/user.module";
import {CourseModule} from "./course/course.module";
import { AuthModule } from './auth/auth.module';
import { JwtModule, JwtService } from '@nestjs/jwt';
import {ConfigModule} from "@nestjs/config";

@Module({
  imports: [ProgramModule,UserModule, AuthModule, CourseModule, JwtModule.register(
      {
        global: true,
        secret: 'unsw-handbookx',
        signOptions: { expiresIn: '15m' },
      }),
      ConfigModule.forRoot({
          isGlobal: true,
      }),
      MongooseModule.forRoot(process.env.MONGO_URI),],

  controllers: [AppController],
  providers: [AppService, JwtService],
})

export class AppModule {}

