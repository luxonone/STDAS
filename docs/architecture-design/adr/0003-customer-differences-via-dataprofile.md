# ADR-0003：客户差异通过 DataProfile / FeatureFlag / Extension 治理

状态：Accepted
日期：2026-05-18

## 背景

STDAS 服务 OSAT 多客户场景。客户差异会出现在文件格式、字段映射、测试阶段、Bin 语义、参数规格、告警规则、分析模板和报表要求中。如果把这些差异写入核心流程或为客户复制服务，系统会快速失控。

## 决策

客户差异必须通过以下机制承载：

- `DataProfile` 和 `ProfileResolutionKey`。
- parser/mapping/spec/alert/template 的版本化绑定。
- FeatureFlags。
- 受控 extension 声明。
- rule fork lineage 和审计。

核心服务不得出现散落的 `if customer == X` 业务分支。不得为单一客户复制服务、复制数据库结构或 fork 独立代码分支。

## 后果

正面：

- 客户差异可版本化、可审计、可追溯。
- 规则变化不会破坏历史 DataVersion 和历史分析结果。
- 客户专属能力可以通过受控扩展演进。

代价：

- 第一版需要建设 ProfileResolution、规则版本和审计模型。
- parser、mapping、spec、alert 和 template 都必须接受 DataProfile 上下文。

## 替代方案

- 为每个客户复制服务：部署表面简单，但长期维护和审计不可控。
- 在核心流程写客户判断：初期快，但会污染核心逻辑。
- 一个客户一个 Profile：无法表达产品、测试类型、测试站点、设备、文件格式和程序版本差异。

## 验证方式

- 新增客户差异时必须能归入 DataProfile、FeatureFlag、rule fork 或 extension。
- 摄入和分析结果必须记录使用的 profile/rule version。
- 代码审查禁止核心业务流程出现客户硬编码分支。
