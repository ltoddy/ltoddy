# Naming（命名）

## MixedCaps

- 多词命名用 camelCase：`MixedCaps`（导出）或 `mixedCaps`（未导出）， **不用** snake_case。
- 常量：导出 `MaxLength`（不是 `MAX_LENGTH`）、未导出 `maxLength`（不是 `max_length`）。不用 `K`/`k` 前缀（如 `kMaxBufferSize`、
  `KMaxUsersPergroup` 都错）。
- 常量按角色命名而非按值命名：`UserNameColumn = "username"` 好；`Twelve = 12`、`UserNameColumn`/`GroupColumn`
  这种纯值别名是坏味道（该用就用，不需要起名）。

## 下划线

一般命名不允许下划线，三个例外：

1. 仅被生成代码 import 的包名（如 `linkedlist_test` 黑盒测试包、`linked_list_service_test` 集成测试包）。
2. `*_test.go` 中的 Test/Benchmark/Example 函数名。
3. 与 OS/cgo 互操作的底层库（如 `syscall`），极罕见。

源 **文件名**不是标识符，可以有下划线。

## Package names

- 只含小写字母和数字；多词包名保持一体全小写：`tabwriter`（不是 `tabWriter`、`TabWriter`、`tab_writer`）。
- 避免被常用局部变量名遮蔽的包名：`usercount` 好于 `count`。
- 避免无信息包名：`util`、`utility`、`common`、`helper`、`model`、`testhelper` 都是坏选择。包名应表达其内容（`spannertest`、
  `elliptic` 好于 `test`、`helper`）。
- import 时重命名的包，本地名也必须遵守上述规则（无下划线、无大写）；同一包在多处重命名时应保持一致。

## Receiver names

- 短（通常 1-2 个字母），是类型的缩写，对所有 receiver 保持一致，不用下划线。
- `func (t Tray)`、`func (w *ReportWriter)`（不是 `this`/`self`）。

## Initialisms

- 缩写整体同大小写：`URL`/`url`、`ID`/`id`、`DB`/`db`，绝不写 `Url`、`Id`。
- 含小写字母的缩写（`DDoS`、`iOS`、`gRPC`）按标准写法；需要切换导出性时整体改大小写：导出 `IOS`/`GRPC`/`DDoS`，未导出 `iOS`/
  `gRPC`/`ddos`。
- 多个缩写连用时每个缩写内部大小写一致：`XMLAPI`（不是 `XmlApi`）。
- `Txn` 正确，`TXN` 错误。

## Getters

- 不要 `Get`/`get` 前缀：用 `Counts` 而不是 `GetCounts`。
- 复杂计算或远程调用可用 `Compute`/`Fetch` 提示耗时。

## 变量名长度

- 名称长度与作用域大小成正比、与使用次数成反比。小作用域（1-7 行）允许单字母；文件级需要多词。
- 优先单字名（`count`、`options`）；需要消歧才加词（`userCount` vs `projectCount`）。
- 不缩写打字：`Sandbox` 好于 `Sbx`。
- 省略类型类词：`userCount` 好于 `numUsers`/`usersInt`；`users` 好于 `userSlice`；同作用域存在两个版本时允许（`ageString` vs
  `age`）。
- 省略上下文已表达的词：`UserCount` 方法内部用 `count`/`users`，不写 `userCount`。
- 常见单字母惯例：`r` 用于 `io.Reader`/`*http.Request`，`w` 用于 `io.Writer`/`http.ResponseWriter`，`i`/`x`/`y` 用于循环/坐标。

## 避免重复

- **包名 vs 导出符号**：包名总是可见，去掉冗余。`widget.NewWidget` → `widget.New`；`db.LoadFromDatabase` → `db.Load`。
- **变量名 vs 类型**：编译器知道类型，`var users int` 好于 `var numUsers int`。
- **外部上下文 vs 局部名**：包名、方法名、类型名、import 路径都提供上下文。在 `package ads/targeting` 中，
  `id := in.GetAdsTargetingID()` 好于 `adsTargetingID := ...`；`type Report struct{}` 好于
  `type AdsTargetingRevenueReport struct{}`。

## 函数/方法名（Best Practices）

- 返回值的函数用名词性名字（`JobName`），做事的用动词（`WriteDetail`）。
- 不重复输入输出类型、receiver 类型、指针性：`Parse(input string)`（在 `yamlconfig` 包中）好于 `ParseYAMLConfig`。
- 同名函数仅类型不同时，类型放末尾：`ParseInt`/`ParseInt64`、`AppendInt`/`AppendInt64`；有明确主版本时可省略：`Marshal()` +
  `MarshalText()`。
- 需要消歧时可加词：`WriteTextTo`/`WriteBinaryTo`。

## Test double 与 helper 包

- 为生产包建测试包：包名 = 生产包名 + `test`（`creditcard` → `creditcardtest`），Bazel 下标记 `testonly`。
- 单一类型的 double 用简短名：`type Stub struct{}`（`creditcardtest.Stub`），好于 `StubService`，远好于
  `StubCreditCardService`。
- 多种行为用行为命名：`AlwaysCharges`、`AlwaysDeclines`。
- 多个类型需要 double 时用 `StubService`/`StubStoredValue`。
- 测试内引用 double 时用前缀区分生产类型：`spyCC`（`creditcardtest.Spy`）好于 `cc`。

## Shadowing

- `:=` 在新作用域内会 **创建新变量**遮蔽外层（shadowing），块结束后恢复原变量——这是常见 bug 来源。
- 有意复用外层变量时用 `=`（stomping），要求类型匹配。
- 不要用与标准库同名变量遮蔽包（局部 `url` 会遮蔽 `net/url`）；选包名时也避免与好变量名冲突。
