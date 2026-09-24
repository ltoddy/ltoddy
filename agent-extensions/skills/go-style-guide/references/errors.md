# Errors（错误处理）与 Commentary（注释）

## 返回错误

- 用 `error` 表示函数可能失败，约定为 **最后一个返回值**。
- 成功时返回 `nil` error。返回 error 时，调用方必须把其他返回值视为未定义（通常为零值，但不能假设）。
- 导出函数返回 `error` 接口而非具体错误类型：具体类型可能把 `nil` 指针包进接口变成非
  nil（见 [Go FAQ nil error](https://golang.org/doc/faq#nil_error)）。
- 接收 `context.Context` 的函数通常应返回 `error`。

## 错误字符串

- 首字母小写（除非以导出名、专有名词或缩写开头），不以标点结尾。错误字符串会出现在更大上下文中。
- `fmt.Errorf("Something bad happened.")` 是坏的；`fmt.Errorf("something bad happened")` 是对的。
- 完整展示消息（日志、测试失败、API 响应）通常首字母大写：`log.Infof("Operation aborted: %v", err)`、
  `t.Errorf("Op(%q) failed; err=%v", args, err)`。

## 处理错误

- 遇到错误要做出明确选择：立即处理并解决 / 返回给调用方 / 极少数情况 `log.Fatal` 或 `panic`。不要用 `_` 丢弃。
- 确有必要忽略时（如文档声明永不失败的 `bytes.Buffer.Write`），加注释说明为何安全。
- **Indent error flow**：错误先处理再继续，正常路径不要缩进进 `else`：

```go
// Good:
if err != nil {
    return err
}
// normal code

// Bad:
if err != nil {
    return err
} else {
    // normal code
}
```

- `if x, err := f(); err != nil { ... } else { ... }` 只在 x 使用范围很短时用；变量用多行就不要用 if-with-initializer
  包住正常逻辑。

## In-band errors

- 不要用 -1、null、空串等哨兵值表错误。用多返回值：`func Lookup(key string) (value string, ok bool)`。
- 这样 `Parse(Lookup(key))` 直接编译不过，强制显式处理。

## 错误结构

- 调用方需要区分错误条件时，给错误 **结构**，不要让他们做字符串匹配。
- 最简单：哨兵值（sentinel）`var ErrDuplicate = errors.New("duplicate")`，用 `==` 或 `errors.Is` 判断（有包装时用
  `errors.Is`）。
- 禁止 `regexp.MatchString('duplicate', err.Error())` 这类字符串匹配。
- 需要额外结构化信息时用错误类型（如 `os.PathError` 的 `Path` 字段）或 `status.Error(codes.X, ...)`。

## 包装错误：`%v` vs `%w`

- `%w` 创建可 `Unwrap()` 的错误链，支持 `errors.Is`/`errors.As`；用于需要调用方程序化检查的场景（应用内部 helper 加上下文）。
- `%v` 丢弃结构信息，仅当：只加非冗余的上下文、只需人类可读消息、或要在系统边界（RPC/IPC/存储）把领域错误翻译成规范化错误空间。
- 添加上下文时避免冗余：`fmt.Errorf("launch codes unavailable: %v", err)` 好；
  `fmt.Errorf("could not open settings.txt: %v", err)` 坏（`os.Open` 错误已含路径）。
- 不要加无信息注释：`fmt.Errorf("failed: %v", err)` 应直接 `return err`。

### `%w` 的位置

- 一般放在错误串 **末尾**（`"context: %w"`），使错误文本从新到旧打印，与链结构一致。
- **例外：哨兵错误**放最前，让错误类别一眼可见：
  `fmt.Errorf("%w: invalid character in header: %v", ErrParseInvalidHeader, err)`。
- 不要写 `fmt.Errorf("%w: err2", err1)`（打印变成从旧到新）或把 `%w` 夹在中间。

## 日志错误

- 返回了错误通常不要再自己 log，让调用方决定（避免重复和 logspam）。
- 注意 PII；`log.Error` 级别昂贵且有 flush，只在可行动时用（error 级别意味着"需要处理"而非"更严重"）。
- 冗长日志用 `log.V(n)`；不要无意调用昂贵函数——`log.V(2).Infof("...%v", sql.Explain())` 在 V 关闭时仍会执行
  `sql.Explain()`，应改写成 `if log.V(2) { ... }`。

## 程序初始化与 panic

- 初始化错误（坏 flag/配置）向上传播到 `main`，用 `log.Exit` 给出可行动消息（不用 `log.Fatal` 的堆栈）。
- 库优先返回 error，不用 panic；仅在内部状态不可恢复的不变量检查时才 `log.Fatal`。
- 不要用 `recover` 避免崩溃（会传播损坏状态）。标准库 `net/http` 对 handler panic 的 recover 被视为历史错误，不要模仿。
- panic 的合理场景：标准库对 API 误用的 panic；包内"永不逃逸出包边界"的实现细节（顶部 `defer recover()` 转成返回
  error，且必须区分自己抛的 panic）；编译器认为不可达的代码（`log.Fatalf` 之后的 `panic("unreachable")`）。
- flag 解析前不要调用 log 函数；`init`/Must 函数中必须终止时可用 panic。

## Must 函数

- 失败即终止程序的 setup helper 命名 `MustXYZ`，只在启动早期/包级初始化用（`regexp.MustCompile`、`template.Must` 模式）。
- 测试里停止当前测试的 helper 用 `t.Fatalf` + `t.Helper()`（如 `mustMarshalAny(t, m)`），可用于表驱动测试的值上下文；不要用在能走常规错误处理的地方。

## 注释与文档

- 所有顶层 **导出**符号必须有 doc comment；未导出但有非显而易见行为/含义的类型和函数也应注释。完整句子，以被描述对象的名字开头（可带冠词）：
  `// A Request represents a request to run a command.`、`// Encode writes the JSON encoding of req to w.`
- doc comment 用 Godoc 渲染给所有使用者，写"为什么"而非复述代码。注释应解释 why，不是 what。
- 完整句子注释首字母大写、带标点；片段（struct 字段行尾注释）可随意。
- 包注释：紧跟 `package` 子句上方无空行；每包只一个文件有；`main` 包用二进制名：
  `// The seed_generator command is a utility that ...`。长包文档可放 `doc.go`。
- 不要过度注释：冗余注释、复述代码的注释是噪音。允许代码自己说话。
- 不重要的参数/字段不必逐一枚举；只注释容易用错或非显而易见的。
- context 语义是隐含的（取消 context 即中断函数），不必复述；行为异常时才显式文档化（返回非 `ctx.Err()`、有额外中断机制、对
  context 有特殊要求）。
- 只读操作默认可并发安全，不必声明；读改写含糊、API 提供同步、或消费用户实现类型时， **必须**文档化并发语义。
- 有显式清理要求必须文档化：`// Call Stop to release the Ticker's associated resources when done.`、
  `// Caller should close resp.Body when done reading from it.`
- 返回的哨兵/错误类型要在文档中说明（含指针 receiver 与否，便于 `errors.Is`/`errors.As`/`cmp`）。
- Godoc 格式：段落间空行；缩进两格为逐字块；大写单行+空行段落自动成标题；runnable example 放在 `*_test.go` 中并写
  `// Output:` 块。
