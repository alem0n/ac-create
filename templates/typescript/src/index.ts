/**
 * 入口点：只做组装与初始化，不承载业务逻辑。
 *
 * 职责契约（高内聚低耦合）：
 * - 读取配置。
 * - 初始化日志。
 * - 完成依赖注入。
 * - 启动应用。
 * - 禁止在此编写业务规则。业务规则属于 services 层。
 */
import { loadConfig } from './config.js';
import { logger } from './logger.js';
import { greet } from './services/index.js';

export async function main(): Promise<number> {
  const appConfig = loadConfig();
  logger.info({ logLevel: appConfig.logLevel }, 'app.start');

  const greeting = greet(appConfig.name);
  logger.info({ greeting }, 'app.done');

  return 0;
}

if (process.env.NODE_ENV !== 'test') {
  void main().then((code) => {
    process.exitCode = code;
  });
}
