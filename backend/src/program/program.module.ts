import { Module } from '@nestjs/common';
import { ProgramController } from './program.controller';
import { ProgramService } from './program.service';
import { MongooseModule } from '@nestjs/mongoose';


@Module({

    controllers: [ProgramController],
    providers: [ProgramService],
})
export class ProgramModule {}
