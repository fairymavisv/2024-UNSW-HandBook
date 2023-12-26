import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { ProgramModule } from './program/program.module';
import { MongooseModule } from '@nestjs/mongoose';
import {UserModule} from "./user/user.module";
import { AuthModule } from './auth/auth.module';

@Module({
  imports: [ProgramModule,UserModule, AuthModule],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
