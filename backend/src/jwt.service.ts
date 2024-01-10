import { Injectable, UnauthorizedException } from '@nestjs/common';
import { JwtService } from '@nestjs/jwt';
import { AppModule } from './app.module';
import * as crypto from 'crypto';
import { get } from 'http';

@Injectable()
export class JwtAuthService {

  constructor(private readonly jwtService: JwtService) {
  }
  /*
  private generateRandomKey() {
    const result = crypto.randomBytes(32).toString('hex');
    return result;
  }
  */
  private secret = 'unsw-handbookx';

  public generateAccessToken(username: string) {
    return this.jwtService.sign({ username }, {secret: this.secret, expiresIn: '3m'});
  }

  public generateRefreshToken(username: string) {
    return this.jwtService.sign({ username }, {secret: this.secret, expiresIn: '7d'});
  }

  public verifyToken(token: string, tokenType: string) {
    try {
      console.log(token);
      console.log(this.secret);
      const decoded = this.jwtService.verify(token, {secret: this.secret});
      console.log(decoded);
      //将decoded中的exp和iat转换为Date类型
      decoded.exp = new Date(decoded.exp * 1000);
      decoded.iat = new Date(decoded.iat * 1000);
      console.log(decoded.exp);
      console.log(decoded.iat);
      return decoded.username;
    } catch (error) {
      if (error.name === 'TokenExpiredError') {
        if (tokenType === 'access') {
          throw new UnauthorizedException('Access token expired');
        } else if (tokenType === 'refresh') {
          throw new UnauthorizedException('Refresh token expired');
        }
      } else {
        throw new UnauthorizedException('Invalid token');
      }
    }
  }
}
   
