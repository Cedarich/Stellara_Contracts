import { Module } from '@nestjs/common';
import { PrismaModule } from '../prisma.module';
import { AbiRegistryController } from './abi-registry.controller';
import { AbiRegistryService } from './abi-registry.service';

@Module({
  imports: [PrismaModule],
  controllers: [AbiRegistryController],
  providers: [AbiRegistryService],
  exports: [AbiRegistryService],
})
export class AbiRegistryModule {}
