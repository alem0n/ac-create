/**
 * dependency-cruiser 分层依赖契约：高内聚低耦合的机器校验
 * 依赖方向单向指向 domain：infrastructure → services → domain
 * 运行：pnpm lint:layers
 *
 * @type {import('dependency-cruiser').IConfiguration}
 */
module.exports = {
  forbidden: [
    {
      name: 'layer-boundary-domain',
      comment: 'domain 层不得导入 services / infrastructure（领域核心须无外部依赖）',
      severity: 'error',
      from: { path: '^src/domain' },
      to: { path: '^src/(services|infrastructure)' },
    },
    {
      name: 'layer-boundary-services',
      comment: 'services 层不得直接导入 infrastructure（须通过接口依赖倒置）',
      severity: 'error',
      from: { path: '^src/services' },
      to: { path: '^src/infrastructure' },
    },
    {
      name: 'no-circular',
      comment: '禁止循环依赖（任何层）',
      severity: 'error',
      from: {},
      to: { circular: true },
    },
  ],
  options: {
    tsPreCompilationDeps: true,
    enhancedResolveOptions: {
      extensions: ['.ts', '.js'],
    },
  },
};
