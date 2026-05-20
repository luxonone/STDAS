# STDAS Page Mockup Prompt Workflow

本文定义 STDAS 前端开发前的页面 mockup 生成流程。它不是最终视觉规范，而是用于在写前端代码前，通过“提示词 -> 图片 -> 审阅 -> 修正”的方式确认 STDAS 的整体页面设计风格。

## 1. 目标

STDAS 的前端 mockup 必须服务于工程分析工作台的风格确认，而不是生成普通后台、展示型官网或泛化 BI 大屏。

本流程的目标是：

- 在前端开发前确认页面信息密度、导航结构、图表/表格比例、筛选方式和工程分析感。
- 让 AI 生成图片前先接受足够完整的业务提示词，避免出现泛化 dashboard。
- 将 SAS 生产系统的业务内容、OneData 的半导体数据分析平台形态、Exensio 的行业级分析平台思路，转译成 STDAS 自己的页面风格。
- 用户确认图片方向后，再把具体风格沉淀为正式前端视觉与交互风格文档。

## 2. 产品范围校准

STDAS 当前版本只服务于本工厂内部测试数据分析场景。虽然本工厂属于半导体封装测试厂，但 STDAS 不是多工厂门户、不是对外客户 Portal，也不是覆盖封装、排程、MES、ERP、WMS 的全厂系统。

当前 mockup 必须遵守：

- 系统只面向本工厂内部测试部门和相关工程角色。
- 登录使用工厂内部账号密码，不使用 SSO，不展示外部身份提供方登录。
- 登录页不展示半导体工厂上下文选择、Environment Profile 选择、客户环境选择或多工厂入口。
- CustomerScope、DataProfile、DataVersion 等概念属于登录后工作台上下文，不应塞进登录表单。
- 登录页必须简洁，只承担身份登录、系统名称、必要安全提示和少量产品识别。
- 登录框默认放在页面右侧；左侧使用抽象化测试数据视觉，不放正式功能预览。
- 登录页不得展示底部信息栏、版权栏、链接组或复杂说明。
- 登录后的页面也不得表现为多工厂平台或外部租户平台。顶部上下文可以出现内部测试部门、FT operation、internal test site 等表达，但不得出现类似 `OSAT1 - Test Factory` 的工厂/租户选择器语义。
- OneData、Exensio、SAS 只作为参考来源，不得作为 STDAS 页面里的产品名、数据源名、页脚文字或可见系统字段。
- 第一阶段只开发 FT 测试系统。BI、SLT 等归入 FT 测试部门范围；CP 晶圆测试系统暂不考虑。
- FT 页面默认不展示 Wafer Count、Wafer 维度、Wafer Map 或 CP 空间分布。只有具体测试数据分析确实使用到 `WaferLot`、`WaferNo`、`X`、`Y` 等可选字段时，才可以在分析工作区作为可选字段出现。
- LotNo 是工厂内部批次号，不等同于 WaferLot。需要同时支持 CustomerLotNo、WaferLot 等字段，但 Overview 和 Lot List 默认以 LotNo 为主。
- 核心业务维度是 `TestType - TestStation`，例如 `FT-FT1`、`FT-FTA`、`BI-BI1`、`BIT-BIT1`，具体命名由客户/产品流程决定，不能写死。
- 同一 LotNo 在同一 TestType-TestStation 维度下可能存在第 1 次、第 2 次、第 3 次测试，也可能由多份测试文件组合而来。组合规则属于后台解析规则。
- 普通工程用户关心 LotEndTime，不关心系统更新时间。列表、图表和筛选优先使用 `LotEndTime`，不使用 `Updated` 作为主时间列。
- 普通工程用户不选择 DataProfile、DataVersion policy 或 user role。DataProfile/DataVersion 是后台治理与追溯概念；user role 由账号配置。
- 系统不对外开放，不出现 public portal、customer portal、external tenant、multi-factory switching 等表达。

## 3. 工作流

STDAS 页面 mockup 采用以下 gate：

1. **提示词生成**
   - 基于 STDAS 文档、本地参考项目和公开资料生成 page mockup prompts。
   - 本步骤只产出提示词，不生成图片。

2. **提示词审阅**
   - 用户审阅提示词是否准确表达 STDAS、SAS 生产经验、OneData/Exensio 行业参考。
   - 不通过则先修改提示词。

3. **图片生成**
   - 使用用户确认后的提示词生成页面图片。
   - 每次生成必须记录使用的 prompt 版本。
   - 图片必须按用户操作路径顺序生成，第一张为 Login / Session Entry。
   - 从第二张开始，后续图片必须把前面已生成且被用户接受或暂定接受的图片作为参考图，延续全局壳层、视觉密度、图表风格、表格样式、控件语言、颜色和状态表达。
   - 如果某张图片被用户明确否决，后续图片不得继续引用该图作为风格参考，必须先修正并重新确认该图或回退到上一张已接受图片。

4. **图片审阅**
   - 用户审阅页面风格、信息密度、业务对象、图表/表格结构。
   - 不通过则修改提示词并重新生成。

5. **风格固化**
   - 用户确认图片后，新增或更新 `frontend-visual-style-baseline.md`。
   - 将最终确认的布局、颜色、密度、组件语义、图表风格、反例和验收规则文档化。

## 4. 图片保存与最终概述

图片确认通过后，必须保存到当前轮次的时间文件夹：

```text
docs/frontend-design/page-mockups/{yyyyMMdd-HHmmss}/
```

规则：

- 未确认或被否决的图片不得放入该时间文件夹。
- 第一张确认通过的图片创建本轮时间文件夹。
- 后续确认通过的图片按生成顺序保存到同一个时间文件夹。
- 文件名必须带顺序号，例如 `01-login.png`、`02-overview.png`、`03-data-explorer-lot-list.png`。
- 最后一张图片确认通过后，必须在同一文件夹生成 `mockup-summary.md`。

`mockup-summary.md` 至少包含：

- 本轮 mockup 时间。
- 已确认图片清单。
- 使用的 prompt 版本。
- 用户确认的整体风格要点。
- 仍需在前端实现阶段注意的 UI/UX 风险。
- 后续需要固化到 `frontend-visual-style-baseline.md` 的具体规则。

## 5. 参考来源

### 5.1 STDAS 内部文档

mockup 必须优先遵守：

- [page-hierarchy-design.md](page-hierarchy-design.md)
- [workbench-design.md](workbench-design.md)
- [ui-ux-constraints.md](ui-ux-constraints.md)
- [frontend-tech-architecture.md](frontend-tech-architecture.md)
- [stdas-domain-primer.md](../domain-knowledge/stdas-domain-primer.md)

这些文档是 STDAS 的事实来源。参考产品只能提供页面表达启发，不能覆盖 STDAS 的 DataVersion、QuerySnapshot、CustomerScope、权限脱敏、查询预算和 Evidence 规则。

### 5.2 本地参考项目

| 参考 | 本地路径 | 用途 | 不采用 |
|------|----------|------|--------|
| SAS | `reference-project/SAS` | 生产级 FT/YM 数据分析内容参考：Lot 查询、Yield Summary、Bin Summary、Parametric DUT/Chip、可选位置/Map 分析、Raw Data Export、Test Step、Tester、Handler、Test Program。 | 不复制旧技术栈、旧 UIkit 风格、过时视觉。 |
| OneData | `reference-project/OneData` | 半导体产品大数据平台与分析工作台参考：产品分析概览、良率/复测/参数分析、图表分析工作区、属性面板、图表工具栏、相关性/ANOVA/箱线图/CDF 等。 | 不复制水印、品牌、具体客户字段和原始界面；FT 第一阶段不默认借鉴 wafer 主视图。 |
| Exensio | `reference-project/Exensio` | 行业级半导体分析平台参考：端到端数据整合、制造/测试/封装数据连接、良率爬坡、root cause、guided analytics、Spotfire 式可视化环境。 | 不复制 PDF Solutions 品牌、受保护界面或营销页。 |

### 5.3 公开资料要点

公开资料只用于理解行业主流页面能力，不用于复制具体视觉。

- [PDF Solutions Exensio Analytics Platform](https://www.pdf.com/products/) 定位为面向 Foundry、IDM、OSAT、Fabless 的半导体数据分析平台，强调连接并分析制造、测试、封装到现场运营的数据，提供 data acquisition、normalization、semantic、big data cloud management、AI/ML 和 Spotfire 可视化。
- [PDF Solutions Manufacturing Analytics](https://www.pdf.com/products/exensio-analytics-platform/modules/manufacturing-analytics/) 强调工程师面对 TB/PB 级数据进行良率爬坡、root cause 定位、guided analytics、产品特征数据、qualification studies、production ramps 和 volume production datasets 分析。
- [PDF Solutions Exensio NPI Yield Engineer](https://www.pdf.com/products/exensio-analytics-platform/modules/manufacturing-analytics/exensio-npi/products-exensio-analytics-platform-modules-manufacturing-analytics-exensio-npi-yield-engineer/) 展示了 parametric yield analysis、wafer mapping、bin/pass-fail/zonal/site analysis、Cpk/yield summary、statistical filtering 等典型分析对象。
- [Gubo OneData](https://www.guwave.com/en/onedata/) 定位为芯片公司的产品大数据平台，覆盖从设计到量产运营，强调设计、实验室、Foundry、OSAT 数据整合，NPI/量产爬坡、良率管理、质量可靠性、供应链透明。
- [Gubo OneData 中文页面](https://www.guwave.com/onedata/) 列出的典型场景包括工程数据分析、量产数据分析、跨阶段数据追溯、车规可靠性筛选、生产过程监控；具体能力包括 Shmoo、PVT Char、变异性分析、良率监控、Fail Bin 监控、晶圆展示、相关性分析、ECID Trace、GDBN、ZPAT、异常批次管理和量产规则运行。

## 6. 参考转译规则

### 6.1 从 SAS 借鉴什么

SAS 是内容参考优先级最高的生产系统。mockup 应借鉴：

- 首页不是“漂亮 dashboard”，而是 Lot / test data 的快速查询入口。
- 数据表格必须包含真实工程字段：Device Name、Lot No.、Test Step、Tester No.、Handler No.、Test Program、Test Time、Yield、Bin1-Bin8。
- Lot 详情需要区分 Merged Yield、Yield of All Tests、Test Data Review、Bin Summary、TT Summary、Param DUT Summary、Param Chip Summary。
- 分析页面必须有导出入口，例如 Yield Summary Export、Raw Yield Summary Export、Parametric Data Export Wizard。
- DUT/Position Map、Bin Map、Parametric distribution 不是装饰图，而是工程分析路径的一部分。WaferLot/WaferNo/X/Y 仅在客户 FT 数据实际提供且分析需要时作为可选字段使用。

### 6.2 从 OneData 借鉴什么

OneData 是半导体数据分析工作台风格参考。mockup 应借鉴：

- 顶部工作区 tabs + 工具栏 + 左侧属性/筛选面板 + 中央图表画布的工作台形态。
- 图表页同时容纳配置面板、统计结果表、散点图、热力图、箱线图、CDF、可选位置图等多种分析对象。
- Overview 页面有产品、测试类型、测试站点、LotNo 维度切换，主区域是良率、Fail Bin、Retest、Parametric Test 分析 tabs。FT 第一阶段不展示 wafer 维度切换。
- 图表必须带有图例、坐标轴、单位、数据范围和工具按钮，不能只是抽象图形。

### 6.3 从 Exensio 借鉴什么

Exensio 是行业平台能力参考。mockup 应借鉴：

- 页面应体现端到端半导体数据平台，而不是单一报表系统。
- 设计要支持制造、测试、封装、OSAT、Fabless 视角下的多源数据分析。
- Analysis Workspace 应体现 guided analytics 和 root cause investigation 的工作流，而不是单个图表页面。
- 告警、规则、Evidence、QuerySnapshot 和 DataVersion 要能连接到 investigation case。

### 6.4 STDAS 自己必须强化什么

STDAS 登录后工程页面必须按场景显式呈现以下一等概念。Overview 可以使用 Overview Snapshot / Snapshot Time 等页面语义，不强行显示 QuerySnapshot；Analysis Workspace、Evidence、Export 和 Investigation 必须显示 QuerySnapshot 或等价稳定引用：

- customer
- product
- LotNo
- CustomerLotNo
- TestType
- TestStation
- TestAttempt
- LotEndTime
- QuerySnapshot
- lot_scope
- query budget
- partial / stale / snapshot / over budget 状态
- permission / redaction 状态
- async job / export 状态
- Evidence version

DataProfile、DataVersion、DataVersion policy 可以在治理、诊断、Evidence、Export 元数据中出现，但不应作为 Overview、Data Explorer、登录页或普通分析入口的主控件。

## 7. 全局视觉方向

### 7.1 应该呈现

- 桌面优先的高密度工程分析工作台。
- 浅色、冷静、专业、可长时间工作的界面。
- 紧凑顶部栏、左侧导航、上下文栏、筛选栏、表格、图表和任务状态入口。
- 图表与表格并重，不能只有 KPI 卡片。
- 每个页面都要看起来像真实工程师会用来定位 LotNo、测试类型、测试站点、LotEndTime、良率、Bin、参数异常和调查证据的系统。
- 使用 1366/1440/1920 宽屏工作台构图；默认生成 16:9 desktop screenshot。
- 文字以短标签为主，允许英文工程标签；不要生成长篇解释文案。

### 7.2 必须避免

- 通用 SaaS admin dashboard。
- 大屏驾驶舱、深色霓虹、地图大屏、装饰性渐变。
- 官网 hero、营销页、卡片堆叠页面。
- 只有 KPI，没有表格和明细。
- 只有图表，没有筛选、DataVersion、QuerySnapshot、权限状态。
- 过度留白、超大字号、低信息密度。
- 随机无意义文本、伪造品牌 logo、复制 Exensio/OneData/SAS 原界面。
- 把 STDAS 画成 MES、ERP、WMS、CRM 或普通 CRUD 后台。
- 把 STDAS 画成多工厂平台、对外客户 Portal、SSO 企业门户或通用身份平台。
- 页面中出现 OneData、Exensio、SAS、OSAT tenant、external customer、multi-factory selector 等可见字段。
- FT 第一阶段页面中默认出现 Wafer Count、Wafer Map、Wafer 主筛选或 CP 空间分析。

## 8. Prompt 使用方式

每次生成图片时，应使用：

```text
[Master Prompt]
+[Page Prompt]
+[Negative Prompt]
+[Accepted Reference Images, if this is not the first page]
```

如需要多轮迭代，保留 prompt 版本号：

```text
mockup-prompt-v0.1
mockup-prompt-v0.2
mockup-prompt-v0.3
```

用户确认图片后，再将最终风格写入正式风格基线文档。

## 9. 图片生成顺序与参考链

图片必须按真实用户操作顺序生成，不能跳着生成。这样可以让前序页面逐步成为后续页面的视觉参考，避免每张图风格漂移。

默认生成顺序：

| 顺序 | 页面 | 参考要求 |
|------|------|----------|
| 1 | Login / Session Entry | 不使用前序图片；建立基础视觉方向。 |
| 2 | Overview | 必须参考 Login，延续产品气质、颜色、字体密度和全局壳层方向。 |
| 3 | Data Explorer / Lot List | 必须参考 Login + Overview，延续导航、上下文栏、表格密度和状态表达。 |
| 4 | Lot Detail / DataVersion Trace | 必须参考前 3 张，延续 Data Explorer 的表格、抽屉、DataVersion 和 lineage 表达。 |
| 5 | Analysis Workspace | 必须参考前 4 张，延续工具栏、筛选面板、表格、图表和状态表达。 |
| 6 | Alerts & Investigation | 必须参考前 5 张，延续工作区布局、Evidence、QuerySnapshot 和 case 面板表达。 |
| 7 | Jobs & Exports | 必须参考前 6 张，延续任务状态、表格、详情面板和导出状态表达。 |
| 8 | System Governance | 必须参考前 7 张，延续系统级页面的壳层、表格、版本状态、diff 和审计表达。 |

后续页面的 prompt 必须显式追加：

```text
Use the previously accepted STDAS mockup images as visual references.
Keep the same global shell, navigation density, typography scale, table styling, chart styling, control language, status badges, color discipline, and engineering workbench feel.
Do not redesign the visual system from scratch.
```

如果用户要求重新定义整体风格，必须从第一张 Login / Session Entry 重新开始，或者明确指定从哪一张已接受图片开始重建参考链。

## 10. Master Prompt

```text
Create a realistic desktop UI mockup for STDAS, an internal semiconductor test data analytics workbench for one factory's testing department.

STDAS serves internal factory test engineering workflows only. The factory is a semiconductor packaging and testing factory, but this system focuses on test data analytics, not packaging operations, MES, ERP, WMS, scheduling, or external customer portal use.

STDAS is not a generic admin dashboard, not a marketing website, not a multi-factory SaaS product, and not an external customer portal. It is a dense internal engineering analysis platform for semiconductor test data, inspired by production FT/YM data analysis tools, OneData-style semiconductor big-data workbenches, and Exensio-style yield management / guided analytics platforms.

The interface must look like a real product used by test engineers, yield engineers, product engineers, quality engineers, and administrators to investigate Lot, optional wafer/die fields when present, bin, parametric test, retest, DataVersion, QuerySnapshot, export jobs, and investigation evidence.

Global UI requirements:
- Desktop-first 16:9 screenshot, 1440px or 1920px wide workbench layout.
- Light professional theme, compact typography, high information density, restrained colors.
- For pages after login, show a top global shell with product name STDAS, current user identity, authorized customer context, high-level test context, time range, and global job queue indicator.
- For pages after login, show left navigation with Overview, Data Explorer, Analysis Workspace, Alerts & Investigation, Jobs & Exports, System Governance.
- Login page is an exception: it must stay simple and must not expose CustomerScope, DataProfile, DataVersion, global job queue, left navigation, SSO, environment profile selector, or factory context selector.
- Login page must not show concrete dashboard previews, concrete table data, concrete KPI cards, bottom information bars, footer links, or anything that looks like a real post-login page.
- After-login pages must avoid duplicated editable context controls. The top shell should show identity, internal test context, and global jobs; page filter bars should own editable customer, product, TestType, TestStation, TestAttempt, and LotEndTime range controls.
- Do not show reference product names such as OneData, Exensio, or SAS in the generated UI.
- Use real FT semiconductor analytics terms: LotNo, CustomerLotNo, TestType, TestStation, TestAttempt, LotEndTime, Site, Bin, Soft Bin, Hard Bin, Yield, Retest, CPK, SPC, Parametric, Test Program, Tester, Handler, QuerySnapshot, Evidence.
- Do not show Wafer Count, Wafer Map, Wafer main filters, DataProfile selector, DataVersion Policy selector, or User Role selector on normal FT pages. DataProfile and DataVersion details may appear only in Governance, diagnostics, Evidence, Export metadata, Lot lineage, or Analysis result metadata.
- Charts and tables must include axes, legends, units, toolbars, row identifiers, and status badges.
- Show data trust state near results: live / cached / snapshot / partial / stale / over budget.
- Show permission/redaction state where relevant.
- Prefer compact panels, split panes, tabs, pinned columns, filters, and toolbar actions.
- The UI should feel like an engineering workbench similar in density to Spotfire/JMP-style visual analytics, but with STDAS-specific data governance and OSAT test context.

Do not copy any brand, logo, watermark, exact layout, or protected UI from reference products. Only borrow the product category, information density, and analysis workflow.
```

## 11. Negative Prompt

```text
Avoid generic SaaS admin dashboard, marketing landing page, hero section, decorative illustration, stock photo, dark neon command center, KPI-only dashboard, empty cards, oversized typography, excessive whitespace, mobile layout, CRM/ERP/MES screen, finance dashboard, map dashboard, multi-factory portal, external customer portal, SSO provider login, environment profile selector, factory context selector on login, OSAT tenant selector, visible OneData/Exensio/SAS product names, Wafer Count on FT Overview, Wafer Map on FT Overview, CP wafer analysis on FT pages, DataProfile selector on normal engineering pages, DataVersion Policy selector on normal engineering pages, User Role selector, copied Exensio UI, copied OneData UI, copied SAS UI, fake official logo, unreadable random text, meaningless chart labels, purely aesthetic charts without axes or legends, charts without FT semiconductor context, tables without LotNo/TestType/TestStation/LotEndTime fields, UI without filters, UI without Overview Snapshot context, UI without error/partial/stale status.
```

## 12. Page Prompts

### 12.1 Login / Session Entry

```text
Generate the STDAS login and session entry page.

Purpose:
An internal factory engineer or administrator signs in with the factory account and password to enter the semiconductor test data analytics workbench. The page should feel like a secure internal engineering system, not a marketing page, SSO portal, external customer portal, or multi-factory platform.

Layout:
- Right-side compact login panel.
- Login fields: factory account, password.
- Optional controls only: remember account checkbox, sign-in button, password reset/help link if visually unobtrusive.
- No SSO button.
- No environment profile selector.
- No factory/customer context selector.
- No DataProfile/DataVersion selection in the form.
- Small internal-system notice area for security or maintenance policy.
- Left side must be abstract and atmospheric, not a real product preview: use simplified signal traces, wafer grid geometry, test data point patterns, yield curve lines, subtle bin-color accents, or layered analytic shapes.
- Do not show concrete post-login widgets such as Lot tables, real charts, KPI cards, wafer maps with labels, job badges, screenshots, or preview dashboards.
- Do not show bottom information bar, footer link group, copyright strip, version strip, or external links.

Visual requirements:
- Light professional style.
- No hero illustration, no stock image.
- Use a subtle technical background pattern such as faint wafer grid or test data rows, but keep it restrained.
- The first screen must communicate "engineering data workbench", not "public website".
- Login page must remain simple and not show too many workbench controls.
- Abstract left-side visual should create the feeling of semiconductor test data analysis without needing to be updated when real functional pages change.
- Logo mark must be refined and project-specific: use a clean abstract mark derived from test data, wafer grid, signal trace, yield curve, chip package, test pins, die grid, or test path.
- If using a chip motif, it must be combined with test-data semantics such as signal traces, probe/test points, die matrix, bin-map pattern, or yield curve; avoid a generic standalone chip icon.
- 当前登录页标识倾向 F 方向：chip package + die grid + yield curve。后续若继续优化，应只做小幅精修，不大幅改变构图语言。
- Avoid generic factory icons, cloud icons, shield icons, lock icons, or random letter logos.

Required visible terms:
STDAS, Test Data Analytics, Factory Account, Password, Secure internal system.
```

### 12.2 Overview

```text
Generate the STDAS Overview page for a semiconductor test data analytics workbench.

Purpose:
Provide a compact operational and engineering summary for the internal FT test department across customer, product, TestType, TestStation, TestAttempt, and LotEndTime range, while preserving freshness, Overview Snapshot, job, and alert status.

This page must feel like a test engineering analysis entry point, not a generic BI/admin dashboard and not an executive KPI screen.

Reference translation:
- Borrow OneData's product analysis overview structure only at the workbench pattern level: product/test-type/test-station context, yield analysis tabs, fail bin trend, tester yield and site yield. Do not borrow wafer context for FT Overview.
- Borrow Exensio's end-to-end yield management idea: connect manufacturing/test/assembly data and guide root cause.
- Borrow SAS's practical FT Yield content: Yield Summary, Bin1-Bin8, Test Program, Tester, Handler, Test Time.

Layout:
- Global shell and left navigation.
- Top shell should not look like a multi-factory or external tenant selector. Use internal wording such as Test Department, FT Operation, Internal Test Site, or current user identity. Do not use visible text like OSAT1 - Test Factory.
- Top shell must not show User Role selector, DataProfile, or DataVersion Policy. Role is account configuration, not a page control.
- Avoid duplicated editable controls. Put editable Customer, Product, TestType, TestStation, TestAttempt, and LotEndTime range in the page context/filter bar.
- Compact KPI strip, not oversized management cards: LotNo Count, Device Count, Avg Yield (FT), Retest Rate, Open Alerts, Running Jobs. Do not show Wafer Count.
- Main area split into:
  1. FT Yield trend chart with clear axes, moving average, test stage label, and Overview Snapshot / stale badge.
  2. Fail HBin / SBin Pareto or stacked trend with top fail bins, legend, cumulative line, and count/% units.
  3. Lot health table with LotNo, CustomerLotNo, Device Name, TestType, TestStation, TestAttempt, Yield, Top Fail Bin, Top Fail Bin %, Tester, Handler, Test Program, LotEndTime.
  4. Alerts and investigation queue side panel with severity, rule version, trigger metric, related LotNo, TestType-TestStation, LotEndTime, and time.
- Secondary tabs: Yield Analysis, Retest Analysis, Parametric Test Analysis, Data Quality.

Required STDAS states:
- Overview should use Overview Snapshot / Snapshot Time wording by default. Do not overuse QuerySnapshot on Overview; reserve QuerySnapshot wording for Analysis Workspace, Evidence, Export, or explicit query results.
- Show partial/stale/cache state if applicable.
- Show link to Data Explorer and Analysis Workspace.
- Footer or data source labels must use STDAS-owned wording such as STDAS Data Platform, Canonical Test Data, or Analytics Snapshot. Do not show OneData, Exensio, SAS, Lakehouse brand names, or reference product names.
- Stronger engineering content is preferred over generic cards: include FT Yield, Fail Bin, Retest, Tester, Handler, Test Program, LotEndTime, rule version, and internal FT test department context.
- Do not show Wafer Count, Wafer map, Wafer main filters, DataProfile selector, DataVersion policy selector, User Role selector, Updated column, or DataVersion column on Overview.

Avoid:
KPI-only dashboard, decorative cards, large empty charts, management-only big screen, generic BI layout, duplicated top/context controls, multi-factory selector, external customer portal, OSAT tenant wording, OneData/Exensio/SAS visible text, Data Source: OneData Lakehouse, Wafer Count, Wafer Map, DataProfile selector, DataVersion Policy selector, User Role selector, Updated column.
```

### 12.3 Data Explorer / Lot List

```text
Generate the STDAS Data Explorer / Lot List page.

Purpose:
Engineers search, filter, compare, and select multiple Lots for analysis. This is a high-density data workbench page, not a simple CRUD table.

Reference translation:
- Borrow SAS's FT Data Explorer fields and export workflow, but use STDAS FT-first terminology.
- Borrow OneData's dense filter + grid + analysis entry pattern, not wafer-oriented context.
- Preserve LotNo, CustomerLotNo, TestType, TestStation, TestAttempt and LotEndTime as first-class user context.

Layout:
- Global shell and left navigation.
- Top context bar with customer, product, TestType, TestStation, TestAttempt, LotEndTime range.
- Filter panel with LotNo, CustomerLotNo, Device Name, TestType, TestStation, Test Program, Tester No, Handler No, Yield range, Bin range, LotEndTime, ingestion status.
- Main table with pinned columns and server-side pagination.
- Required columns: selection checkbox, LotNo, CustomerLotNo, Device Name, TestType, TestStation, TestAttempt, Yield, Bin1-Bin8 summary, Test Program, Tester No, Handler No, LotEndTime, File Count, Permission State.
- Toolbar actions: Add to Analysis Workspace, Compare Lots, Export Yield Summary, Export Raw Data, Save Filter, Open QuerySnapshot.
- Selection summary must distinguish current page selection, manual Lot selection, and current filter result selection.
- Right drawer or side panel for selected LotNo summary, test dimension differences and file-combination status.

Required STDAS states:
- Show query id or overview/list snapshot id.
- Show data freshness and partial result warning.
- Show over-budget path: narrow filter or run async export.
- Do not show DataProfile selector, DataVersion policy selector, Updated column, Wafer Count or Wafer Map by default.

Avoid:
Simple admin table, table without selection semantics, fake customer data without engineering fields, DataProfile selector, DataVersion Policy selector, Updated as primary time, Wafer Count, Wafer Map.
```

### 12.4 Lot Detail / DataVersion Trace

```text
Generate the STDAS Lot Detail and DataVersion Trace page.

Purpose:
An engineer opens one Lot and inspects test steps, runs, files, yield, bin summary, parametric summaries, optional position maps, lineage, and DataVersion history.

Reference translation:
- Borrow SAS's Lot Detail structure: Merged Yield, Yield of All Tests, Test Data Review, Bin Summary, TT Summary, Param DUT Summary, Param Chip Summary, and optional DUT/position map links when data fields exist.
- Add STDAS file-combination, parser/mapping/spec rule lineage for diagnostics; keep DataProfile/DataVersion out of the main user header.

Layout:
- Header with LotNo, CustomerLotNo, Device Name, Product, Customer, TestType, TestStation, TestAttempt, current status, LotEndTime.
- Summary strip: Yield, Retest Rate, Fail Bin Top 3, Device Count, Test Duration, LotEndTime, file count.
- Tabs:
  1. Yield of Tests
  2. Bin Summary
  3. Parametric Summary
  4. DUT / Position Map, only if position fields exist
  5. Files & Lineage
  6. DataVersion History
- Main area should show a dense test-step table plus chart area.
- Internal lineage timeline with parse attempt, parser profile, mapping profile, spec profile, committed/ready/superseded states.
- File lineage table: raw file, file hash, parser version, mapping version, ingestion job, created time.

Required STDAS states:
- Show whether current view is latest committed, historical version, snapshot, or stale.
- Show action to add this Lot to Analysis Workspace.
- Show permission/redaction state for restricted files or fields.

Avoid:
Generic detail page, single card layout, no lineage, no test data tables.
```

### 12.5 Analysis Workspace

```text
Generate the STDAS Analysis Workspace page.

Purpose:
Engineers analyze multiple Lots across Yield, Bin, Parametric, Retest, SPC, correlation, ANOVA, optional position/wafer fields when present, and root cause investigation. This is the most important workbench page.

Reference translation:
- Borrow OneData's graph analysis workspace: top toolbar, left properties/settings panel, central multi-panel charts, statistic result table, scatter plot, heatmap, box plot, CDF, and optional position map patterns when STDAS data provides those fields.
- Borrow Exensio's guided analytics/root-cause platform idea.
- Borrow SAS's actual semiconductor analysis objects: Bin Summary, Param DUT/Chip Summary, Multi Param Probability Plot, optional DUT/position map, Raw Data export.

Layout:
- Global shell and left navigation.
- Workspace title with workspace name, saved state, query snapshot id, and frozen data reference state.
- Top toolbar: Run Query, Save Workspace, Save as Template, Export, Add Evidence, Create Case, Reset Zoom.
- Left configuration panel:
  - Lot Scope
  - Analysis Type
  - X/Y Parameter
  - Group By: Site / Tester / Handler / optional WaferLot when present / Test Step
  - DataVersion policy as an advanced query metadata control, not a normal top-shell selector
  - Query budget estimate
  - Options API loading/error/permission states
- Central canvas with resizable panels:
  1. Query summary and result trust banner.
  2. Result table: p-value, Pearson r, Spearman p, R squared, n, fail bin, CPK.
  3. Scatter plot with regression line and selected brush region.
  4. Heatmap or correlation matrix.
  5. Box plot / CDF plot.
  6. Optional position map / die map small multiples only when WaferLot/WaferNo/X/Y fields exist.
- Bottom or right evidence panel showing selected anomaly, linked Lot, DataVersion set, and Add to Investigation Case.

Required STDAS states:
- Every result panel must show QuerySnapshot, DataVersion set, partial/stale/cache state.
- Show old request protection concept: "Applied filters" vs "Pending changes".
- Show async path when query exceeds budget.
- Show exit protection for unsaved workspace.

Avoid:
Single chart page, BI dashboard, generic analytics page, charts without table, table without chart linkage, no DataVersion.
```

### 12.6 Alerts & Investigation

```text
Generate the STDAS Alerts & Investigation page.

Purpose:
Engineers triage yield/bin/parameter/data quality alerts, open investigation cases, capture evidence, and preserve QuerySnapshot/DataVersion context.

Reference translation:
- Borrow Exensio's root cause and guided analytics direction.
- Borrow OneData's production monitoring and abnormal Lot management concepts.
- Borrow SAS's production fields: Lot No, Test Step, Tester, Handler, Test Program, Yield, Bin.

Layout:
- Global shell and left navigation.
- Alert filter bar with customer, product, severity, rule version, time range, status, trigger context.
- Alert list table with severity, rule name, rule version, Lot, Product, Test Step, metric, threshold, triggered value, DataVersion, status, owner.
- Investigation workspace split view:
  - Left: alert timeline and case status.
  - Center: evidence board with chart snapshots, table excerpts, optional position map when relevant, query summary.
  - Right: root cause notes, linked jobs, related Lots, permission/redaction state.
- Case header must show case id, state, owner, created time, latest evidence version.

Required STDAS states:
- Evidence must show QuerySnapshot id, DataVersion set, generated time, evidence version.
- Recomputed evidence must be visibly a new evidence version.
- Permission changes must show hidden/masked/unauthorized states.

Avoid:
Simple ticket list, generic notification center, evidence without frozen data context.
```

### 12.7 Jobs & Exports

```text
Generate the STDAS Jobs & Exports page.

Purpose:
Users track ingestion, analysis, export, retry, replay, and expired jobs across the system.

Reference translation:
- Borrow SAS's practical export needs: Yield Summary Export, Raw Yield Summary Export, Parametric Data Export Wizard.
- Add STDAS async job lifecycle, query snapshot, DataVersion, permission and expiry rules.

Layout:
- Global shell and left navigation.
- Job filters: job type, status, customer, product, owner, created time, DataVersion, request id.
- Main job table with status badges: queued, running, succeeded, failed, canceling, canceled, expired, retry scheduled, dead letter.
- Required columns: job id, job type, owner, customer, product, progress, current stage, created time, updated time, completed time, expires at, query snapshot id, retry count.
- Right detail panel for selected job:
  - stage timeline
  - request parameters summary
  - DataVersion set
  - export file format
  - estimated rows
  - masked/unmasked status
  - download / retry / replay / cancel actions
  - failure reason and next action

Required STDAS states:
- Export download must show permission re-check state.
- Expired file must show metadata but no dead link.
- Retry must explain whether it reuses query snapshot or re-resolves latest committed.

Avoid:
Generic task manager, no DataVersion, no expiry, no failure diagnostic.
```

### 12.8 System Governance

```text
Generate the STDAS System Governance page.

Purpose:
Admins manage CustomerScope, DataProfile, parser/mapping/spec rules, feature flags, templates, permissions, and audit trails for OSAT multi-customer data governance.

Reference translation:
- Borrow OneData's spec management and design-to-test data traceability concept.
- Borrow Exensio's semantic data model and rule-driven analytics platform idea.
- Preserve STDAS governance around DataProfile, ProfileResolutionKey, versions, impact analysis, rollback, audit.

Layout:
- Global shell and left navigation.
- Governance sub-navigation: Customers, DataProfiles, Profile Resolution, Parser Profiles, Mapping Profiles, Spec Rules, Alert Rules, Feature Flags, Users & Permissions, Audit.
- Main DataProfile list with customer, product, test type, test station, equipment type, file format, program, version, state, effective time, owner.
- Detail/edit panel showing draft/published/deprecated states.
- Diff viewer comparing current draft vs published version.
- Impact analysis panel showing affected customer/product/test type/test station/Lot count/DataVersion/rule version.
- Validation panel with blocking errors and warnings.
- Audit timeline with publisher, approver, timestamp, reason.

Required STDAS states:
- Publishing must show immediate/scheduled/new-data-only effect.
- Deprecated/delete must show downstream dependencies.
- Concurrent edit conflict must be visible.
- Permission state must distinguish admin UI access from business data scope.

Avoid:
Generic settings page, simple CRUD forms, no version diff, no impact analysis.
```

## 13. Prompt Review Checklist

提示词进入图片生成前，必须回答：

- 是否明确 STDAS 是半导体测试数据分析工作台，而不是普通后台？
- 是否体现 SAS 生产系统里的 Lot、Yield、Bin、Parametric、可选位置/Map、Export 内容？
- 是否体现 OneData 式产品分析、图表分析、属性面板和多图联动？
- 是否体现 Exensio 式 end-to-end、root cause、guided analytics 和良率管理平台？
- 是否按页面场景显式要求 CustomerScope、DataProfile、DataVersion、QuerySnapshot、Evidence，而不是把治理概念放到普通页面主控件？
- 是否要求高密度表格和图表并重？
- 是否禁止泛化 admin dashboard、营销页、大屏驾驶舱和 KPI-only 页面？
- 是否足够具体到页面区域、字段、状态和操作？
- 是否规定了图片生成顺序，并要求后续图片参考前面已接受图片？
- 是否避免把 STDAS 误画成多工厂平台、外部客户 Portal、SSO 门户或全厂 MES/ERP 系统？
- 登录页是否足够简单，只保留工厂账号密码登录和必要安全提示？

## 14. Image Review Checklist

图片生成后，审阅重点不是像素级完美，而是判断方向是否可作为前端风格探索基础：

- 看起来是否像工程师每天使用的生产分析系统？
- 页面是否有足够的数据密度，而不是漂亮但空？
- 图表、表格、筛选、工具栏、状态是否同时存在？
- 是否能一眼看出这是半导体测试/Yield/Bin/Parametric/Lot 系统？
- 是否有 STDAS 的 DataVersion、QuerySnapshot、CustomerScope、Evidence 等独有概念？
- 是否保留 SAS 生产系统的业务价值，而不是只借鉴旧 UI？
- 是否避免复制 Exensio/OneData/SAS 的品牌和具体界面？
- 是否适合作为 React + TypeScript 前端实现前的风格确认材料？
- 如果这不是第一张图，是否明显延续了前序已接受图片的壳层、颜色、表格、图表、控件和状态语言？
- 登录页是否没有 SSO、环境 Profile、工厂上下文选择和过多工作台功能？
- 通过确认的图片是否已保存到本轮时间文件夹，并使用顺序号文件名？

## 15. 当前图片状态

`page-mockups/` 目录下已有图片为第一轮生成结果，当前状态为 **未采纳 / 待替换**。这些图片不得作为 STDAS 最终视觉风格基线。

下一轮必须先完成本提示词文档审阅，再生成新图片。
