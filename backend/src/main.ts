import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { SwaggerModule, DocumentBuilder } from '@nestjs/swagger';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
<<<<<<< HEAD

=======
  const config = new DocumentBuilder()
      .setTitle('Backend API')
      .setDescription('The backend API description')
      .setVersion('1.0')
      .addTag('develop')
      // 全局响应
      .build();
  const document = SwaggerModule.createDocument(app, config);
  SwaggerModule.setup('api', app, document);
>>>>>>> demoA
  await app.listen(3000);
}
bootstrap();
