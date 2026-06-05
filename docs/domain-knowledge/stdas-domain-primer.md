# STDAS 背景知识总览

本文帮助新加入的开发者、AI Agent、产品和测试人员快速理解 STDAS 的业务场景、核心对象、用户工作流和第一阶段边界。它是领域知识入口，不替代产品愿景、领域模型、前端设计或后端设计。

## 1. STDAS 是什么

STDAS（Semiconductor Test Data Analysis System）是面向 OSAT 封装测试工厂的半导体测试数据分析工作台和数据平台。

它要把分散在测试文件、MES、采集目录和工程师经验中的信息组织成可查询、可分析、可追溯的工程工作台。核心目标不是“把数据库搬到浏览器上”，而是让工程师快速判断哪些 LotNo 需要关注、异常来自哪里、是否和测试类型、测试站点、测试次数、机台、Handler、Site、Bin、参数、程序版本或 LotEndTime 有关，并把关键结论沉淀为 Evidence、Export 或 Investigation Case。

## 2. STDAS 不是什么

STDAS 不替代 MES。MES 仍负责批次流转、扣留、解扣、工单和生产执行。STDAS 只读同步 MES 的批次、产品、设备、扣留和工艺上下文，用于分析展示，不发起生产控制动作。

STDAS 不替代 Excel 或通用 BI。Excel 和 BI 适合临时分析和通用报表，但 STDAS 必须内置半导体测试语义，例如 LotNo、客户 LotNo、测试类型、测试站点、测试次数、Test File、Hard Bin、Soft Bin、Datalog、Retest、Cpk、SPC、Site、Handler、Program Version、LotEndTime、QuerySnapshot 和 Evidence。

DataProfile、DataVersion、ParserProfile、MappingProfile 等是系统解析、治理和追溯所需的内部概念。普通工程用户希望看到的是正确的客户、产品、LotNo、测试类型、测试站点、良率、Bin、参数、LotEndTime 和异常原因，而不是被迫理解底层解析配置或数据版本策略。

STDAS 也不是展示型官网或普通 CRUD 后台。它是高密度工程分析工作台，设计重点是上下文连续、数据可信、版本可追溯、多 Lot 对比和异常调查。

## 3. 工厂与业务背景

STDAS 面向本工厂内部测试数据分析场景。工厂内部测试业务大体分为 CP 测试和 FT 测试两大类：CP 面向晶圆形态，FT 面向成品芯片形态。本项目第一阶段只开发 FT 测试系统；BI、SLT 等在内部归入 FT 测试部门范围，CP 测试系统暂不考虑。

FT 系统的默认工程入口不应出现 Wafer 相关主视图或 KPI。部分客户的 FT 测试数据可能包含 `WaferLot`、`WaferNo`、`X`、`Y` 等字段，这些字段可以在具体测试数据分析中作为芯片唯一标识或规则判断条件使用，但不是每个客户都会提供，也不应成为 FT 开班检查或 Lot 查询的默认上下文。

OSAT 场景的关键特点：

- 多客户：不同客户可能有不同文件命名、字段含义、Bin 定义、规格限、测试流程、测试站点命名和报告格式。
- 多测试流程：同一客户不同产品可能有不同测试类型和测试站点定义，例如 FT1/FT2/FT3、FTA，或 BI/BIT 等客户专属命名。
- 多平台：同一测试类型可能来自不同 ATE、Handler、BI 平台或客户定制格式。
- 多版本：测试程序、规格、规则、解析器和数据版本会随时间变化。
- 强追溯：分析结论必须能追溯到测试文件、Lot、Run、设备、程序版本、解析规则和数据版本。

这些特点决定 STDAS 不能只按“表格 + 图表”设计，而要从第一天承认客户差异、版本差异和追溯要求。

## 4. 第一阶段范围

第一阶段以 FT 测试系统为主，优先实现成品芯片测试 Summary 数据入库、查询和分析。BI、SLT 等内部归入 FT 部门的测试类型可按客户/产品流程逐步纳入；CP 晶圆测试系统暂不实现。Datalog 参数分析作为增强能力，在客户提供 Datalog 且解析规则明确后启用。

第一阶段纳入：

- FT 部门测试数据上传和自动采集。
- 按客户、产品、测试类型、测试站点、设备、文件格式和程序解析数据。
- Summary 文件解析。
- 可选 Datalog 解析。
- LotNo 列表、Lot 详情、测试类型-测试站点维度、测试次数、Run/File/解析记录追溯。
- 良率趋势、Bin Pareto、重测分析、UPH、基础统计控制图。
- MES 只读同步批次上下文、产品信息、过账记录、LotEnd 判断所需信息和扣留状态。
- 分析工作台、调查证据、导出和模板能力的基础设计。

第一阶段不纳入或只预留：

- CP 晶圆测试系统。
- 跨 CP 到 FT 的完整追溯链。
- MES 写操作、扣留、解扣或设备控制。
- 通用 BI 式自由 SQL 或无限制拖拽分析。
- 未经过查询预算控制的大范围明细扫描。

## 5. 核心用户

| 用户 | 主要目标 | 典型问题 |
|------|----------|----------|
| 测试工程师 | 快速定位异常 Lot、机台、Bin、参数和重测问题 | 今天哪些批次异常？这个 Bin 为什么升高？是不是某台机台或 Site 问题？ |
| 工艺/产品工程师 | 分析参数分布、规格边界、Cpk/SPC 和长期趋势 | 参数是否漂移？规格是否过紧？产品版本变更是否影响良率？ |
| 管理层 | 查看 KPI、良率、产出、异常批次和扣留风险 | 当前良率是否稳定？哪些客户、产品或站点风险最高？ |
| 系统管理员 | 配置客户、平台、采集任务、解析规则和权限 | 新客户文件怎么接入？哪些用户可以看哪些客户数据？ |

## 6. 关键领域对象

| 对象 | 含义 | STDAS 关注点 |
|------|------|--------------|
| Customer | 客户或委托方 | 决定权限范围、文件格式、规则和数据隔离 |
| Product | 产品型号或产品系列 | 常用于筛选、趋势比较和规格规则选择 |
| LotNo | 工厂内部按内部规则生成的批次号，工程分析的核心对象 | 与实际 WaferLot 不必相同；良率、Bin、重测、MES 扣留和追溯的主线 |
| CustomerLotNo | 客户定义或客户侧识别的批次号 | 有时与工厂 LotNo 或 WaferLot 一致，有时不同 |
| WaferLot / WaferNo / X / Y | 部分客户数据提供的晶圆或坐标字段 | FT 页面默认不展示为主维度；仅在具体分析中作为可选识别和规则判断字段 |
| TestType | 客户/产品定义的测试类型，例如 FT、BI、BIT、SLT 等 | 命名以客户要求为准，不能写死 |
| TestStation | 客户/产品定义的单个测试站点，例如 FT1、FT2、FT3、FTA、BI1 等 | 与 TestType 共同构成核心分析维度 |
| TestAttempt | 同一 LotNo 在同一 TestType-TestStation 下的测试次数，例如第 1 次、第 2 次、第 3 次 | 用于初测、重测、最终合并口径和良率判断 |
| LotEndTime | LotNo 在某个 TestType-TestStation 维度下的实际作业结束时间 | 工程用户关注的时间字段，优先来自测试数据或 MES 过账记录 |
| Run | 一次测试运行或一次批次测试记录 | 区分测试次数、不同机台、不同程序版本或不同文件组合 |
| Test File | 原始测试文件或压缩包 | 入库事实来源，必须保留文件指纹和解析记录 |
| Summary | 批次级或分组级汇总数据 | 第一阶段主要分析数据来源 |
| Datalog | DUT/参数级明细数据 | 参数分布、Cpk、SPC、箱线图、相关性分析的数据来源 |
| Bin | 测试分类结果 | Hard Bin / Soft Bin 用于失效分类、Pareto 和趋势分析 |
| Site | ATE 多 Site 并行测试位置 | 用于判断并行测试一致性和硬件问题 |
| Handler/Socket | 分选机和接触位置 | 用于定位接触不良、机械问题和位置相关异常 |
| Program Version | 测试程序版本 | 用于解释良率或参数突变 |
| DataProfile | 数据解析、映射、规格、规则和模板的组合配置 | 内部治理概念，普通工程用户不需要在日常页面理解或选择 |
| DataVersion | 某批数据参与查看、分析或导出时使用的内部稳定数据版本 | 内部追溯概念；普通页面优先展示业务口径、LotEndTime、文件组合和快照状态 |
| QuerySnapshot | 一次分析查询的冻结语义 | 确保历史 workspace、case、export 不随最新数据静默变化 |
| Evidence | 调查案例中的证据 | 绑定 QuerySnapshot、DataVersion 和生成时间 |

## 7. 测试阶段概念

FT（Final Test）是封装后成品测试，是第一阶段重点。FT 通常关注最终良率、Bin 分类、重测回收、机台/Handler/Site 差异、测试时间和参数分布。

在本工厂语境中，BI、SLT 等成品形态相关测试归入 FT 测试部门范围，但客户可能使用自己的命名和流程。系统不能假设所有客户都叫 `FT1`、`FT2`、`BI1`，也不能把 BI 强行写成固定模型；测试类型和测试站点必须来自客户/产品专属流程配置。

术语统一：

- `TestType` 表示客户/产品定义的测试类型或测试阶段，例如 `FT`、`BI`、`BIT`、`SLT`。
- `TestStation` 表示该测试类型下的具体测试站点或流程节点，例如 `FT1`、`FT2`、`FTA`、`BI1`。
- `Site` 表示 ATE 并行测试位置，不等于 `TestStation`。
- 新增 API、配置、事件和前端状态不得使用 `site_type` 表达测试类型；如旧文档或迁移数据出现 `site_type`，应按 `test_type` 解释并在修订时改名。

CP（Circuit Probe / Wafer Sort）发生在晶圆阶段，关注 Wafer、Die、Probe Card 和空间分布。当前第一阶段不实现 CP 分析，但模型不应把 FT 假设写死到所有对象上。

BI（Burn-In）是老化测试，关注高温、通电、时间、BIB/Board/Slot、失效率和可靠性筛选。BI 数据和 FT 数据结构差异较大，通常不能简单套用 FT Summary 模型。

SLT（System Level Test）是系统级测试，更接近真实使用场景，关注复杂功能、长时间运行、系统组合和边缘问题。

## 8. 数据从哪里来

STDAS 的数据来源主要有四类：

1. 测试机台或共享目录产生的测试文件。
2. 工程师手动上传的压缩包或测试文件。
3. MES 提供的批次、产品、设备、扣留和工艺上下文。
4. 系统治理配置，例如客户配置、解析规则、Bin 定义、规格限、分析模板和权限。

测试文件是测试事实来源，MES 是生产上下文来源，治理配置决定如何解释这些数据。三者不能互相替代。

## 9. 数据进入 STDAS 的典型流程

```text
测试文件产生
-> 文件采集或手动上传
-> 文件登记和指纹去重
-> 识别客户、产品、测试类型、测试站点、平台、格式、程序版本
-> 解析 DataProfile
-> 解压和解析 Summary / Datalog
-> 数据校验和标准化
-> 生成或提交 DataVersion
-> 补充 MES 上下文和 LotEnd 过账记录
-> 按 LotNo + TestType + TestStation + TestAttempt 组合测试文件
-> 构建可查询的 Lot / Summary / Bin / Parameter 数据
-> 工程师查询、分析、保存证据或导出结果
```

这个流程中最重要的是：文件解析失败不能污染稳定数据；重新解析同一文件应产生可追溯记录；同一 LotNo 在同一 TestType-TestStation 维度下可能由多份测试数据组合而来，组合规则属于客户/产品专属解析规则；用户可见时间优先展示 LotEndTime，而不是系统更新时间。

## 10. Summary 与 Datalog

Summary 通常是批次级、Run 级、Site 级或 Bin 级汇总结果。它适合第一阶段快速支撑良率、Bin、重测、UPH 和批次列表。

Datalog 是更细粒度的参数测量明细，通常包含 DUT、测试项、测量值、上下限、单位、Site、时间或状态。它适合参数直方图、Cpk、SPC、箱线图、相关性、异常点和规格边界分析。

Summary 可以回答“哪个 Lot 或 Bin 异常”。Datalog 才能回答“哪个参数、哪个 Site、哪个位置、哪个分布形态导致异常”。

## 11. 常见分析问题

| 问题 | 主要数据 | 常用视图 |
|------|----------|----------|
| 今天哪些 LotNo 良率异常？ | Summary + MES | 开班检查入口、Lot 查询、良率趋势 |
| 某个 Bin 为什么升高？ | Summary Bin | Bin Pareto、Bin Trend、Lot 对比 |
| 是不是机台问题？ | Summary + Equipment | 按机台良率、机台热力图、UPH 趋势 |
| 是不是 Handler 或 Site 问题？ | Summary Site / Handler | Site 良率、Handler/Socket 对比、位置图 |
| 重测是否有效？ | INI / RT / MER Summary | Retest Funnel、Bin Recovery、MER Trend |
| 参数是否漂移？ | Datalog | Histogram、SPC、Box Plot、Cpk Trend |
| 是否程序版本引起？ | Summary/Datalog + Program Version | 版本分组趋势、变更前后对比 |
| 扣留批次有什么共同特征？ | MES Hold + Summary | Hold Lot List、Yield/Bin 对比 |

## 12. 指标口径

良率（Yield）一般表示 Pass 数量占 Tested 数量比例。FT 场景中需要区分初测良率、重测后良率和最终良率。不同客户、产品或测试站点可能有不同命名和合并口径，UI 和 API 中必须明确口径。

Bin 表示测试结果分类。Hard Bin 通常更接近最终分类，Soft Bin 通常更接近测试程序内部分类。分析时不能只展示 Bin 号，还要尽量展示 Bin 名称、Pass/Fail 属性、数量、比例和趋势。

重测（Retest）用于判断初测失败后再次测试是否通过。高重测回收率可能提示接触问题、测试稳定性问题或边界条件问题；低回收率更可能是真实缺陷。

参数指标用于判断制程和测试稳定性。Cpk/Cp、SPC 控制图、分布形态、箱线图和异常点都依赖 Datalog 或等价明细数据。

## 13. MES 在 STDAS 中的角色

MES 提供生产上下文，不是测试数据本身。STDAS 从 MES 获取 Lot 基本信息、产品属性、过账记录、LotEnd 判断所需信息、设备/程序上下文、扣留状态和扣留原因。

STDAS 的边界是只读分析：

- 可以展示扣留状态。
- 可以帮助工程师分析扣留批次的测试特征。
- 可以提示 MES 信息缺失或过期。
- 不触发扣留、不解除扣留、不修改 MES 批次状态。

## 14. 多客户和多平台差异

不同客户可能在文件命名、压缩包结构、Summary 字段、Datalog 行格式、Bin 定义、产品命名、程序命名、规格限和分析模板上不同。

因此，解析和分析不能写成单一客户硬编码。客户差异应进入 DataProfile、ParserProfile、MappingProfile、SpecProfile、RuleSet、Feature Flag 或受控 extension，而不是散落在核心流程里的 `if customer == ...`。

## 15. 数据可信度

工程师是否信任 STDAS，取决于系统是否说清楚数据状态：

| 状态 | 含义 |
|------|------|
| raw | 原始文件已登记但未解析 |
| parsed | 文件已解析出中间结构 |
| normalized | 数据已映射到标准模型 |
| committed | 数据版本已提交，可用于分析 |
| partial | 只有部分文件、部分 Lot、部分参数或部分结果可用 |
| stale | 当前展示结果不是最新上下文 |
| snapshot | 展示的是历史查询快照 |
| hidden/masked | 数据存在但因权限或规则被隐藏/脱敏 |
| failed | 解析、查询、导出或任务失败 |

缓存、导出和图表不能改变数据事实。缓存只加速读取，事实来源仍然是原始文件、数据库元数据、DataVersion 和 QuerySnapshot。

## 16. 工程师典型工作流

开班检查：进入后续前端设计确认的开班检查入口，查看当前客户、产品、时间范围下的异常 Lot、良率趋势、Top Bin 和扣留状态。如果没有异常，保持监控；如果有异常，进入调查。

异常调查：从异常 LotNo 或告警进入调查视图，系统保留客户、产品、LotNo、测试类型、测试站点、测试次数、LotEndTime 和快照上下文。工程师依次查看良率、Bin、机台、Site、Handler、重测和参数视图。

深入工作流：当预设调查不够时，工程师进入后续前端设计确认的分析工作流，自定义多 Lot 对比、参数选择、图表布局和筛选条件。关键结果可保存为 Evidence 或导出。

知识沉淀：资深工程师把常见调查路径保存为分析模板，后续相似问题可复用，不再依赖口头经验。

## 17. 领域不变量

- 同一分析结果必须能追溯到具体 LotNo、TestType、TestStation、TestAttempt、Run、File、解析规则和内部数据版本。
- 同一个 LotNo 在同一 TestType-TestStation 维度下可能有多次测试、重测、第三次测试或重新解析记录，不能简单覆盖历史。
- 不同客户的文件格式、Bin 语义和规格规则不能混用。
- 不同客户或同一客户不同产品的测试类型、测试站点命名和文件组合规则不能混用。
- FT 系统默认不把 Wafer 作为主维度；WaferLot/WaferNo/X/Y 仅在数据实际提供且分析场景需要时使用。
- MES 信息缺失不能阻断测试数据入库，但必须标识上下文不完整。
- MES 扣留状态是分析上下文，不是 STDAS 可执行的生产控制动作。
- Summary 和 Datalog 是不同粒度的数据，不能用 Summary 伪造参数明细。
- 历史证据和导出必须引用当时的数据版本或查询快照，不能静默改成最新结果。
- 权限隐藏和真实无数据必须区分。

## 18. 最小词汇表

| 术语 | 简要解释 |
|------|----------|
| ATE | 自动测试设备，执行电性测试 |
| Handler | FT 中负责上下料、分选和接触的设备 |
| Site | ATE 并行测试通道或位置 |
| DUT | Device Under Test，被测器件 |
| Lot | 批次，工程分析主对象 |
| LotNo | 工厂内部批次号，默认主对象 |
| CustomerLotNo | 客户侧批次号 |
| WaferLot | 晶圆批次号，FT 中为可选字段 |
| TestType | 客户/产品定义的测试类型 |
| TestStation | 客户/产品定义的单个测试站点 |
| TestAttempt | 测试次数，通常用于初测、重测或第三次测试 |
| LotEndTime | LotNo 在 TestType-TestStation 维度下的作业结束时间 |
| Run | 一次测试运行或测试阶段记录 |
| INI | Initial Test，初测 |
| RT | Retest，重测 |
| MER | 最终合并或最终有效测试结果，具体口径依客户/平台定义 |
| Hard Bin | 硬件或最终分 Bin 分类 |
| Soft Bin | 测试程序内部细分 Bin |
| Datalog | 参数或 DUT 级测试明细 |
| Summary | 汇总测试结果 |
| Cpk | 制程能力指数，衡量分布相对规格限的能力 |
| SPC | 统计过程控制，用于监控过程稳定性 |
| SYL/SBL | 统计良率限 / 统计 Bin 限 |
| Hold | MES 中的批次扣留状态 |
| DataProfile | 内部数据解析、映射、规格和规则配置集合，不是普通用户日常页面概念 |
| DataVersion | 内部可追溯稳定数据版本，不作为普通页面主显示字段 |
| QuerySnapshot | 一次分析查询的冻结上下文 |
| Evidence | 调查案例中保存的证据 |
