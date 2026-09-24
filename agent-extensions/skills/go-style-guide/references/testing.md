# Testing（测试）

## 有用的失败消息

测试失败应能 **不读源码**就定位问题。消息需包含：

- 失败原因
- 出错的输入
- 实际结果（got）
- 期望结果（want）

标准格式：`YourFunc(%v) = %v, want %v`。消息里要 **标识函数**（`Translate(%q, %q, %q) = %q, want %q`）和 **标识输入**
（输入复杂/大时给用例起名字描述，打印 description）。

- **got 在 want 前**：先打印实际值再打印期望值。
- **`t.Error` 优先于 `t.Fatal`**（Keep going）：一次运行打出所有失败。只有后续检查无意义时才 `t.Fatal`（如编码后解码）。
- 多行 diff 前加换行，并给出方向图例：`diff (-want +got):\n%s`。
- 复杂输出打 diff（`cmp.Diff`）而不是完整打印两个值。

## 禁止断言库

- 不要创建 assertion helpers（把校验和失败消息绑在一起、接收 `*testing.T` 并调 `t.Fatal`/`t.Errorf` 的库）：
  `assert.IsNotNil(t, "obj", obj)` 是反模式。
- 用标准 `cmp`（`cmp.Equal`/`cmp.Diff`）+ `fmt` + `t.Errorf`。比较逻辑应该 **返回值/error** 给测试用，而不是吞掉
  `testing.T`。
- 测试只能基于标准库 `testing` 包；第三方测试框架和断言库都不允许。

## 比较

- `==` 只适用于标量、可比较 struct/interface；slice 等用 `cmp.Equal`。
- 优先 `cmp`（Go 团队维护，可配置，结果稳定）；不用 `reflect.DeepEqual`（对未导出字段/实现细节敏感）；`pretty` 有坑（不区分 nil
  与空 slice、不适用 proto）。
- proto 消息比较必须带 `protocmp.Transform()` 选项。
- 比较整个结构（全字段深比较），不要手写逐字段比较；含无关字段时例外。
- 避免比较不稳定的输出（如 `json.Marshal` 的字节序列）：先解析再比语义。
- **测试错误语义**：不要用字符串比较检查错误类型（会成为 change detector）；用 `errors.Is` 或 `cmpopts.EquateErrors`
  。只想关心"有没有错"时，`wantErr bool` + `!= nil` 最简单，不要引入 `cmpopts.AnyError`。

## 表驱动测试

- 大量用例用相似逻辑时用表驱动。失败消息必须标识输入， **不要用行号**（`Failed on case #%d` 是反模式）。
- 给表加 `name`/`desc` 字段并打印；成功用例省略错误相关字段（利用零值）。
- 测试用例 struct 字面量 **用字段名**（尤其 >20-30 行、同类型相邻字段、想省略零值字段时）。
- 不同逻辑的用例拆成多个测试函数（GoTip #50）；简单错误检查可并入现有表，但别让循环体里长出条件分支。
- 零值/nil 输入本身就是测试点时要显式写字段。

## Subtests

- 子测试互相独立，不能依赖执行顺序或共享状态（要能 `-run` 单独跑）。
- 子测试名像函数标识符：简洁、可打、易过滤。避免斜杠（test filter 特殊含义）、避免散文式长描述；长描述放 `desc` 字段用 `t.Log`
  或并进失败消息。
- 表驱动 + `t.Run` 时，单个用例失败用 `t.Fatal`（结束该子测试，继续下一个）；不用 `t.Error` + `continue`。

## Test helpers

- helper = 做 setup/cleanup 的函数；传入 `*testing.T` 时必须调 `t.Helper()`（让失败定位到调用处）。参数顺序：context（如有）→
  `*testing.T` → 其余。
- helper 失败通常意味着无法继续（环境失败），用 `t.Fatal` 并给出描述（`Setup failed: could not write pak0 asset: %v`）。
- helper 里没有可能失败的操作时，就别收 `t` 参数。
- **不要在子 goroutine 里调 `t.Fatal`/`t.FailNow`**（`testing.T` 文档明确禁止）；用 `t.Error` + `return`。`t.Parallel`
  不改变这一限制。

## 测试结构

- 同一包测试放 `foo_test.go`（`package foo`），可访问未导出标识符；黑盒测试用 `package foo_test`。
- setup 尽量贴近具体测试用例（`mustLoadDataset(t)` 按需调用），不要用 `init()` 做全局 setup——
  `go test -run TestRegression682831` 不该背上慢初始化。
- 公共 setup 昂贵 + 只需部分测试 + 无需 teardown 时可用 `sync.Once` 摊销；所有测试都要且要 teardown 才考虑自定义
  `TestMain`（首选不应该）。
- 集成/组件测试用 **真实传输层**（生产 client + test server），不要手写 imitating 客户端的 double。
- 测试辅助/测试 double 包：生产包名 + `test`（`creditcardtest`），test double 命名见 `naming.md`。

## 其他

- 不要在测试里引入领域特定语言；校验用 Go 本身。
- `t.Cleanup` 注册清理函数（Go 1.14+），比手动 defer 更适合 helper。
- 大输出打 diff、字符串用 `%q`、小 struct 用 `%+v`。
- 修改全局状态后必须恢复，保证测试 hermetic。
