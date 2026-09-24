# Language & API Design（语言细节与 API 设计）

## 复合字面量

- 外部包类型初始化 **必须写字段名**（字段位置/全集不是 public API）：`csv.Reader{Comma: ',', Comment: '#'}`，禁止
  `csv.Reader{',', '#', 4}`。
- 包内类型字段名可选，但字段多时仍应写。
- 花括号匹配：多行字面量收尾 `}` 与开头同缩进；行尾以逗号结尾、`}` 下一行。
- Cuddled braces（`{` 紧跟前文）：slice/array 字面量中仅当缩进匹配且内部是字面量或 proto builder 时允许。
- 省略重复类型名：`[]*Type{{A: 42}, {A: 43}}` 好于重复 `&Type{A: ...}`；map 同理。用 `gofmt -s` 简化。
- 省略零值字段：`db.Options{BlockSize: 1 << 16, ErrorIfDBExists: true}` 好于把所有零值字段列出来。

## nil slice

- `nil` 与空 slice 在 `len`/`cap`/`range`/`append` 下行为一致。
- 局部变量声明 `var t []string`（nil 初始化），不要 `t := []string{}`。
- API 不要迫使调用方区分 nil 与空 slice；用 `len(s) == 0` 判断空，不要 `s == nil`。

## 函数签名与调用

- 签名尽量保持单行（避免 indentation confusion）；太长的参数列表用 option struct 或 variadic options 重构，不要硬折行。
- 调用不因行长而随意折行；按语义分组折行是允许的。
- 不要给参数加行内注释（`42, // Port`）；用 option struct 或文档。
- 长字符串字面量不要为了行长拆开拼接（`"..."+ "..."` 是坏的）；格式串后换行、参数按语义分组。

## 条件与循环

- `if` 不折行；长条件把布尔操作数提取成局部变量。
- 不重复提取：`uid := user.GetUniqueUserID()` 后 `db.UserIsAdmin(uid) || ...` 好于重复 `user.GetUniqueUserID()`。
- `switch`/`case` 保持单行；过长时 `case` 后换行并空行分隔。
- 比较时变量在左、常量在右（`result == "foo"`，禁 Yoda 风格 `"foo" == result`）。
- `switch` 内 `break` 是冗余的（case 自动 break）；要跳出外层 `for` 用 label 的 `break loop`。
- **提取公共子表达式**是首选：`inTransaction := db.CurrentStatusIs(...); keysMatch := ...`。

## 复制与 receiver

- 不要复制含不可复制字段的 struct（`sync.Mutex`、`bytes.Buffer`）：`b2 := b1` 会别名底层数组。方法集在 `*T` 上的类型不要按值复制。
- 不用指针参数只图省字节：`*string`、`*io.Reader` 是坏的，直接传值。大 struct 和 proto 消息用指针。
- **Receiver 类型**（正确性优先于速度）：
    - 需要修改 receiver → 指针。
    - 含不可安全复制字段（mutex 等）→ 指针。
    - slice receiver 若不 reslice/reallocate → 值。
    - 大 struct/array → 指针。
    - map/function/channel → 值。
    - 小值类型（无可变字段无指针）→ 值。
    - 拿不准 → 指针。一个类型的 method 尽量统一指针或统一值。

## 变量声明

- 非零值初始化用 `:=`（`i := 42` 好于 `var i = 42`）。
- 零值声明表示"空值、准备后用"：`var coords Point`、`var primes []int`（好于 `Point{X: 0, Y: 0}`、`[]int(nil)`）。
- 需要指针时：`new(pb.Bar)` 或 `&pb.Bar{}`；proto 消息必须用指针（满足 `proto.Message`）。
- 复合字面量用于已知初始元素：`primes := []int{2, 3, 5}`。
- size hint 仅在性能敏感且已知大小时用（`make([]Node, 0, 16)`）；预分配过大反而浪费内存。
- 尽量指定 channel 方向：`func sum(values <-chan int) int`，编译器能抓发送/接收错误。

## Goroutine 生命周期

- 启动 goroutine 时要让"何时/是否退出"清晰；泄漏的 goroutine 阻塞在 channel 上不会被 GC 回收。
- 用 `sync.WaitGroup` + `defer wg.Done()` + `wg.Wait()` 保证不超出函数生命周期；或用 `context.Context` 统一取消。
- 不要 spawn 了就不管（`go process(item)` 是坏的）：undefined behavior、难测试、可能泄漏。
- 优先 **同步函数**（同步返回结果、返回前完成回调/channel 操作）；调用方需要并发时自己 `go f()`。加并发易、去并发难。

## 接口

- **没有真实需要就不要创建接口**；先聚焦具体行为和具体实现。
- 不要为抽象/测试包装 RPC client（用真实 transport）；不要仅为测试导出 test double 实现。
- 接口要小；消费者定义接口（只含自己用到的方法），生产者仅在接口即产品（如 `io.Writer`、proto 生成接口）时导出。
- **接收接口、返回具体类型**：调用方可用完整方法集；返回接口仅用于封装（如 `error`）、多实现选择（factory/strategy）、打破循环依赖。
- 未导出接口也值得文档化（常是内部粘合逻辑）。

## Generics

- 满足业务需求即可用；但"只有一种实例化类型"时先用非泛型，别为算法/数据结构套泛型。遵循 least mechanism。
- 导出泛型 API 必须充分文档化，鼓励 runnable example。
- 不要用泛型发明 DSL / 错误处理框架；测试里尤其警惕断言库。

## Context

- `context.Context` 永远是函数 **第一个参数**；test helper 中 context 在 `*testing.T` 之前。
- 不要把 context 存进 struct；作为参数传给每个需要的方法。唯一例外是签名必须匹配 stdlib/第三方接口。
- 禁止自定义 context 类型或签名里用别的接口：`func F(ctx context.Context, ...)`。
- 应用数据放参数/receiver/global，不要放 context value（除非真正属于 context）。
- 不要在链中段自己 `context.Background()`；从调用方拿。入口（main/init/test 的 `t.Context()`）才创建基础 context。
- context 不可变，可共享给多个调用。

## Imports

- 分组顺序：① 标准库 ② 其他（项目/vendored）③ proto（`foopb "path/to/foo_go_proto"`）④ side-effect（`_ "..."`）。
- proto import 重命名带 `pb`/`grpc` 后缀，名称去下划线：`foosvcpb "path/to/foo_service_go_proto"`；新代码用描述性名字（
  `pushqueueservicepb`）而非 `xpb`。
- `import .` 禁止（看不清来源）。
- `import _` 只允许 main 包或测试中；例外：`embed` 包、绕 nogo 检查。
- 无信息包名（`util`、`v1`）可重命名，但克制；本地名一致。

## 常用库与杂项

- 用 `%q` 格式化字符串而非手动 `\"%s\"`（空串/控制字符时 `""` 一目了然）。
- 新代码用 `any` 而非 `interface{}`。
- 类型定义 `type T1 T2`（新类型）与类型别名 `type T1 = T2`（引用现有类型）分清；别名只用于迁移，不要滥用。
- 密钥/随机 token 用 `crypto/rand`，禁用 `math/rand`（可预测）。
- flag 只在 `package main` 定义；库用 Go API 配置而非 CLI。flag 名 snake_case、变量 camelCase。
- 字符串拼接：少量用 `+`；带格式用 `fmt.Sprintf`（写 `io.Writer` 用 `fmt.Fprintf` 不建临时串）；逐步构建用 `strings.Builder`
  （线性时间）；复杂模板用 `text/template`；多行常量串用反引号。
- 全局状态：库不要强迫客户端依赖 package-level 可变状态（服务定位器、回调注册、thick-client singleton
  都是反模式）；让客户端创建实例并显式传依赖。全局状态安全仅当：逻辑常量、行为无状态、不泄漏到程序外部、无行为可预测性预期。
- 避免导入为副作用传播到库包。

## 文档与格式化速记

- 注释过长时在 80-100 列折行（非硬性）；单行塞大段文字是坏的。
- 不要在缩进换行处引入与代码块对齐的折行（indentation confusion）。
- `log.Info(v)` 与 `log.Infof("%v", v)` 等价，无格式时用前者。本指南中 `log` 指 glog 风格日志库（`log.Fatal` 带堆栈、
  `log.Exit` 不带），不是标准库 `log`。
