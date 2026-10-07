/**
 * 配置层：统一加载环境变量，供组合层注入。
 *
 * 职责契约：仅做配置读取与校验。不包含业务逻辑。不访问外部服务。
 */
import * as process from 'node:process';

export interface AppConfig {
  readonly name: string;
  readonly logLevel: string;
}

export function loadConfig(): AppConfig {
  return {
    name: process.env['APP_NAME'] ?? 'world',
    logLevel: process.env['LOG_LEVEL'] ?? 'info',
  };
}
