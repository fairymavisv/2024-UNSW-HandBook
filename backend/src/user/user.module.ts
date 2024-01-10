import { Module } from '@nestjs/common';
import { UserController } from './user.controller';
import { UserService } from './user.service';
import { MongooseModule } from '@nestjs/mongoose';
import { userSchema } from './user.model';
import { ConfigModule } from '@nestjs/config';

@Module({
    imports: [
        ConfigModule.forRoot({
            isGlobal: true,
        }),
        MongooseModule.forFeature([{ name: 'User', schema: userSchema }]),
        MongooseModule.forRoot(process.env.MONGO_URI),
    ],
    controllers: [UserController],
    providers: [UserService],
})
export class UserModule {}
