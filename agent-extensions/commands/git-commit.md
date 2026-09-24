---
name: git-commit
description: 根据已暂存的 Git 更改创建上下文感知的提交
---

# 任务

根据当前 git 暂存区（staged）的内容创建一次提交。本命令不接受任何参数，一切以暂存区的实际内容为准。

## 执行步骤

1. 并行运行以下命令收集信息：
    - `git status`：查看已暂存的文件列表（不要使用 -uall，大仓库下可能导致内存问题）
    - `git diff --cached`：查看暂存区的具体改动内容
    - `git log --oneline -10`：了解仓库近期的提交风格

2. 分析改动：
    - 概括改动的性质：add（新功能）、update（现有功能增强）、fix（修复 bug）、refactor、docs、test 等
    - 检查暂存区是否有敏感文件（.env、credentials.json 等），若有请先提醒用户并暂停，不要提交

3. 撰写 commit message：
    - 简洁（1-2 句），聚焦 "为什么" 而非 "做了什么"
    - 遵循仓库现有的提交风格（参考第 1 步的 git log 结果）

4. 创建提交：
    - 只提交已暂存的内容，不要额外 `git add` 未暂存的改动
    - 使用 HEREDOC 传递 commit message，例如：

      ```
      git commit -m "$(cat <<'EOF'
      <commit message>
      EOF
      )"
      ```

5. 验证：运行 `git status` 确认提交成功、工作区状态符合预期

## 要求

- 如果暂存区为空（没有任何已暂存的改动），直接告知用户，不要创建空提交
- 如果 pre-commit hook 失败：修复问题后创建新的 commit，不要使用 --amend
- 除非用户明确要求，不要 push 到远程仓库
