/**
 * 日志模块：唯一允许创建与配置 pino 实例的地方。
 *
 * 职责契约：业务模块一律从本文件导入 logger。
 * 禁止各模块自行创建 logger 实例或输出 console。
 */
import pino from 'pino';

export const logger = pino({
  level: process.env['LOG_LEVEL'] ?? 'info',
  formatters: {
    level(label) {
      return { level: label };
    },
  },
});
