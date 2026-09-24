---
name: git-create-branch
description: 根据当前暂存区 diff 生成分支名并创建分支
---

# 任务

根据当前 Git 暂存区（staged）的 diff 内容生成分支名，并创建后切换到该分支。如果暂存区为空，直接告知用户，不要创建分支。

## 执行步骤

1. 运行以下命令：
    - `git diff --cached`
    - `git branch --list`

2. 根据当前暂存区的 diff 内容生成简洁、明确的分支名：
    - 仅根据 staged diff 命名
    - 使用 `feat/`、`fix/`、`refactor/`、`docs/`、`test/` 等合适前缀
    - 使用小写字母和连字符
    - 如果名称已存在，生成新的不冲突名称

3. 使用 `git checkout -b <branch-name>` 或 `git switch -c <branch-name>` 创建并切换分支。
