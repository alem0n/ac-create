/**
 * 领域模型示例（模板占位，替换为实际领域概念）。
 */

export class Greeting {
  constructor(private readonly subject: string) {}

  render(): string {
    return `Hello, ${this.subject}!`;
  }
}

export function buildGreeting(subject: string): Greeting {
  return new Greeting(subject);
}
