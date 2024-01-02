import { Injectable } from '@nestjs/common';
import { JwtService } from '@nestjs/jwt';
import * as crypto from 'crypto';
import { get } from 'http';

@Injectable()
export class JwtAuthService {
  private secret: string;

  constructor(private readonly jwtService: JwtService) {
    this.secret = generateRandomKey();
  }

  async generateToken(payload: any): Promise<string> {
    console.log('secret: ', this.secret);
    console.log('payload: ', payload);
    return this.jwtService.sign(payload, { secret: this.secret });
  }

  async verifyToken(token: string): Promise<any> {
    try {
      return this.jwtService.verify(token, { secret: this.secret });
    } catch (error) {
      return null;
    }
  }
}

function generateRandomKey() {
  const result = crypto.randomBytes(32).toString('hex');
  // 如果result为undefined，那么就报错，不然就打印出来
  if (result === undefined) {
    throw new Error('generate random key error');
  }
  console.log('random key: ', result);
  return result;
}

