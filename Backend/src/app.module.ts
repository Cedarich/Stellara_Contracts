import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { RedisModule } from './redis/redis.module';
import { VoiceModule } from './voice/voice.module';
import { DatabaseModule } from './database/database.module';
import { StellarMonitorModule } from './stellar-monitor/stellar-monitor.module';

@Module({
  imports: [
    DatabaseModule,
    RedisModule,
    VoiceModule,
    StellarMonitorModule,
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
