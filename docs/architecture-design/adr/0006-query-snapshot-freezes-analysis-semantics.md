# ADR-0006：采用 QuerySnapshot 冻结分析语义

状态：Accepted
日期：2026-05-18

## 背景

多 Lot 分析、导出、workspace 和 investigation case 依赖 DataVersion。若 latest committed 在查询后变化，历史链接可能静默展示不同结果，导致分析不可复现。

## 决策

分析查询执行时必须生成或引用 `QuerySnapshot`，冻结：

- CustomerScope。
- lot_scope。
- data_version_policy。
- 实际参与查询的 DataVersion 集合。
- 查询条件摘要。
- 查询预算和降级结果。
- 结果引用。

图表、表格、导出、workspace、case 和 evidence 必须引用 QuerySnapshot 或等价稳定引用。

## 后果

正面：

- 分析结果可复现。
- evidence 不会被静默重算覆盖。
- 异步重试和导出可以复用同一查询语义。

代价：

- 查询接口和结果存储都必须处理 snapshot id。
- latest committed 刷新必须生成新的 QuerySnapshot。

## 替代方案

- 只保存查询条件：无法防止 DataVersion 变化。
- 只保存结果文件：缺少查询语义和审计上下文。

## 验证方式

- 分析响应必须返回 query id、query hash 或 query snapshot id。
- 历史 workspace/case/export 打开时默认读取固化 DataVersion。
- evidence 重算必须生成新的 evidence version。
