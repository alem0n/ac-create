/**
 * 测试约定：测试置于 tests/。文件名使用 <module>.spec.ts。测试镜像源码树。
 * 变更纪律：新增/修改任何源码，必须同步新增/更新测试。
 */
import { describe, expect, it, vi } from 'vitest';

import { loadConfig } from '../src/config.js';
import { logger } from '../src/logger.js';
import { main } from '../src/index.js';
import { buildGreeting } from '../src/domain/greeting.js';
import { greet } from '../src/services/index.js';

describe('config', () => {
  it('loads defaults', () => {
    const config = loadConfig();
    expect(config.name).toBe('world');
    expect(config.logLevel).toBe('info');
  });
});

describe('entrypoint', () => {
  it('returns exit code 0 and emits structured log', async () => {
    const infoSpy = vi.spyOn(logger, 'info');
    await expect(main()).resolves.toBe(0);
    const doneCall = infoSpy.mock.calls.find((call) => call[1] === 'app.done');
    expect(doneCall?.[0]).toMatchObject({ greeting: 'Hello, world!' });
    infoSpy.mockRestore();
  });
});

describe('domain', () => {
  it('renders greeting', () => {
    expect(buildGreeting('world').render()).toBe('Hello, world!');
  });
});

describe('services', () => {
  it('greets', () => {
    expect(greet('world')).toBe('Hello, world!');
  });

  // 断言行为而非复述实现：services 层输出应与 domain 规则一致
  it('uses domain rule', () => {
    expect(greet('agent')).toBe(buildGreeting('agent').render());
  });
});
