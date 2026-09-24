# Agent Extensions

该目录用于存放可复用的 agent 扩展。

## Setup

执行以下命令可将 `skills/` 与 `commands/` 安装到 `~/.agents/` 目录：

```bash
./setup.sh
```

安装过程会创建 `~/.agents/skills/` 与 `~/.agents/commands/`，并将本目录下对应内容复制进去；同名文件会被覆盖。

## Commands

`commands/` 用于保存可直接复用的工作流提示词。

- `code-review.md`：审查当前 Git 暂存区中的代码变更，检查明显 bug、AGENTS.md 规则符合性和相关上下文。
- `git-commit.md`：根据当前 Git 暂存区内容生成提交信息并创建提交，不会主动添加未暂存文件。
- `git-create-branch.md`：根据当前 Git 暂存区 diff 生成分支名，并创建后切换到新分支。

## Skills

`skills/` 用于保存面向特定领域的技能定义及参考资料。

- `go-style-guide/`：Go 代码风格指南，用于编写或审查 Go 代码，覆盖命名、错误处理、测试、文档和接口设计等内容。
- `rust-api-guidelines/`：Rust API 设计指南，用于设计或审查 Rust crate 的公共 API，覆盖命名、文档、类型安全、兼容性和错误设计等内容。
