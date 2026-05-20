# 摄入流水线

## 跨服务流水线

```text
stdas-gateway
  -> upload / register file

data-pipeline-service
  -> Validate File
  -> Store Raw
  -> Create Ingestion Job
  -> Detect File Envelope
  -> Build ProfileResolutionKey
  -> Resolve DataProfile / ParserProfile / MappingProfile / SpecProfile
  -> Publish FileValidated / ProfileResolved
  -> Parse To Staging
  -> Normalize By Mapping
  -> Validate Business Rules
  -> Commit Canonical Data
  -> Create DataVersion
  -> Publish CanonicalDataCommitted

analytics-service
  -> Build Aggregates
  -> Publish AggregatesBuilt
  -> Evaluate Alerts
  -> Publish AlertRaised / AlertCleared

workflow-service
  -> Track job state
  -> Retry / compensate / mark DataVersion Ready
```

## 步骤契约

每个步骤必须定义：

- 输入。
- 输出。
- 状态。
- 错误码。
- 是否可重试。
- 幂等键。
- 日志字段。

## Parser 边界

Parser 只负责把输入文件转换为标准解析结果，不直接写核心业务表。

```rust
trait TestFileParser {
    fn parser_id(&self) -> ParserId;
    fn parser_version(&self) -> ParserVersion;
    async fn parse(&self, input: ParseInput) -> Result<ParsedDataset, ParseError>;
}
```

生产路径不依赖 parser 内部 `supports()` 扫描，而是通过 ProfileResolutionKey 查询 DataProfile，再由 DataProfile 绑定到 ParserProfile 和 ParserRule，最后由 registry 获取明确版本的 parser：

```text
ProfileResolutionKey
  -> DataProfile
  -> ParserProfile
  -> ParserRule(parser_id, parser_version)
  -> ParserRegistry.get(parser_id, parser_version)
  -> parser.parse(input)
```

ProfileResolutionKey：

```text
customer_code
product
test_type
test_station
equipment_type
file_format
program_name
program_version
effective_time
```

客户/测试类型/测试站点/设备专用 parser 可以存在，多个 DataProfile 也可以共享同一个 ParserRule。需要隔离变更时，可以从共享规则复制分叉出新的 ParserRule 独立维护。所有 parser 必须注册在 ParserRegistry 中，并由 DataProfile 选择。`data-pipeline-service` 的主流程不直接引用客户专用 parser 类型。

## 幂等规则

| 作业 | 幂等键 |
|------|--------|
| ingest file | file hash + customer + product + test type + test station + equipment type + file format |
| parse file | raw file hash + parser id + parser version + profile resolution key |
| normalize data | staging dataset + data profile version + mapping rule version + spec rule version |
| commit core data | lot run key + data version |
| build aggregate | aggregate type + data version |
| evaluate alert | rule id + data version |
| export result | query hash + data version + requester |

## 文件安全

- 限制文件大小。
- 校验扩展名和 magic bytes。
- 限制压缩包层数、文件数量和总解压大小。
- 原始文件只读归档。
- 临时目录隔离。
- 解析超时。
- 解析失败保留诊断信息。
