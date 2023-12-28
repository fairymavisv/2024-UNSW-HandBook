import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { ProgramModule } from './program/program.module';
import { MongooseModule } from '@nestjs/mongoose';
import {UserModule} from "./user/user.module";
import {CourseModule} from "./course/course.module";
import { AuthModule } from './auth/auth.module';
import { JwtModule } from '@nestjs/jwt';

@Module({
  imports: [ProgramModule,UserModule, AuthModule, CourseModule, JwtModule.register(
      {
        secret: 'secret',
        signOptions: { expiresIn: '1d' },
      })],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}

