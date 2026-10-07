/**
 * services 层：用例编排。
 *
 * 高内聚低耦合契约：
 * - 只可导入 domain 层与抽象接口。
 * - 不得导入 infrastructure 层的具体实现（通过依赖倒置交互）。
 * - 不得直接访问外部 IO（数据库/HTTP/文件系统）。
 */
import { buildGreeting } from '../domain/index.js';

export function greet(subject: string): string {
  const greeting = buildGreeting(subject);
  return greeting.render();
}
