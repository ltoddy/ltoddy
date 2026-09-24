---
name: go-style-guide
description: >-
  Go 代码风格速查与评审清单，覆盖命名、格式化、错误处理、文档、测试、
  语言细节、接口设计、context、import 分组和可维护性。
  当用户要求写 Go 代码、审查 Go 代码风格、命名 Go 标识符、处理 Go 错误、
  编写 Go 测试、设计 Go API，或提到 Go style、Go 风格、golang 代码规范、gofmt 时使用。
---

# Go Style Guide

这是一套面向 Go 代码编写与审查的风格建议，重点覆盖独立执行时容易做错的规则。

## 风格原则（按优先级排序）

1. **Clarity**：代码的目的和理由对读者清晰。从读者角度而非作者角度评估。
2. **Simplicity**：用最简单的方式达成目标。避免不必要的抽象（least mechanism：优先核心语言结构 > 标准库 > 自建依赖）。
3. **Concision**：高信噪比。减少重复代码、多余语法、不透明命名。
4. **Maintainability**：易于未来修改。避免隐藏关键细节（如 `=` 与 `:=`、`!` 的细微差异）。
5. **Consistency**：与周边代码一致。风格未规定之处跟随同文件/同包的既有立场；但一致性不能凌驾于上述原则之上。

## 必须遵守的核心规则

- **Formatting**：所有 Go 源码必须符合 `gofmt` 输出。结构性简化用 `gofmt -s`。
- **MixedCaps**：多词命名用 camelCase，不用 snake_case。常量导出用 `MaxLength`（不是 `MAX_LENGTH`），未导出用 `maxLength`
  。局部变量视为未导出。
- **行长度**：无固定行宽。行太长优先重构而不是折行；不要为了缩进一致而硬折行（indentation confusion）。
- **错误处理**：用 `error` 作为最后的返回值；错误字符串首字母小写、不以标点结尾；错误先处理再继续（indent error
  flow），正常路径不要缩进到 `else` 里。

## 快速评审清单

审查 Go 代码时逐项检查；需要更完整规则或示例时，按需加载对应 reference。

- [ ] 命名符合 Go 惯例（无下划线、无 `Get` 前缀、initialisms 大小写正确、receiver 短且一致、避免重复）→ [naming.md](references/naming.md)
- [ ] 错误处理正确（返回 `error` 而非裸值、错误字符串格式、`%v` vs `%w` 选择、sentinel/结构错误）→ [errors.md](references/errors.md)
- [ ] 注释与文档规范（导出符号必须有 doc comment、注释以符号名开头、包注释位置）→ [language.md](references/language.md)
- [ ] 测试写法规范（有用失败消息、禁用断言库、`t.Helper`、表驱动测试、`t.Error` vs `t.Fatal`）→ [testing.md](references/testing.md)
- [ ] 语言细节（复合字面量、nil slice、receiver 指针/值、goroutine 生命周期、接口设计、context、import 分组）→ [language.md](references/language.md)

## 常见陷阱（最容易犯错的地方）

这些是违背直觉、智能体默认容易做错的规则：

- **`Get` 前缀**：`GetCounts` 是错的，应为 `Counts`。只有 HTTP GET 等本身含 "get" 的概念才保留。
- **Initialisms 大小写**：`url`、`Id`、`Api` 是错的。`URL`/`url`、`ID`/`id`、`API`/`api` 才对。导出 `URLPony`，未导出 `urlPony`。
- **错误字符串**：`fmt.Errorf("Something bad happened.")` 是错的（首字母大写 + 句号）。应为 `"something bad happened"`。
- **断言库**：不要为测试创建 assertion helpers（`assert.NotNil(t, x)` 这类）。用 `cmp` + 标准 `t.Errorf`。
- **命名结果参数**：不要为了省函数内变量而命名返回值；小函数才允许 naked return。
- **`%q` 优先**：格式化字符串用 `%q` 而非手动 `\"%s\"` 包裹。
- **`nil` vs 空 slice**：局部变量声明 `var t []string` 而非 `t := []string{}`；用 `len(s) == 0` 判断空，不要用 `== nil`
  ；不要设计 API 让调用方区分 nil 和空 slice。
- **`panic`**：不用 panic 做常规错误处理。程序初始化错误用 `log.Exit`/`log.Fatal`（不是标准库 log，指 glog 风格），不可达代码才用
  panic。
- **shadowing**：`:=` 在 if 块内会创建新变量遮蔽外层（Go 1.22+ 循环变量问题也要留意）；stomping（直接 `=` 复用外层变量）是允许的。
- **自定义 context 类型**：禁止。永远用 `context.Context`，且必须是函数第一个参数。不要把 context 存进 struct。
- **`import .` 和 `import _`**：`import .` 禁止；`import _` 只允许在 main 包或测试中。
- **`any` vs `interface{}`**：新代码用 `any`。
- **断言测试用 `switch` 的 `break`**：多余，Go 的 case 自带 break；要跳出外层 for 用 label。

## 输出

给出评审意见或代码修改时，遵循规范并尽量附带 Good/Bad 对照示例；指出违规时说明违反了哪条规则。不确定的边界情况（如接口是否该导出、generics
是否过度设计）按「最少机制」原则倾向于更简单的方案。
