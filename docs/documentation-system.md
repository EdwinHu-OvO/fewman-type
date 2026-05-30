# 文档架构

本文说明项目文档如何分层，以及人和 AI 应该如何读取。

## 分层

```text
README.md      面向普通用户
AGENTS.md      面向 AI agent 和维护者的入口规则
docs/          可共享的专题文档
memory/        本机私有长期记忆，默认不提交
```

## 各层职责

| 层级 | 是否提交 | 读者 | 内容 |
| --- | --- | --- | --- |
| `README.md` | 是 | 普通用户 | 项目介绍、安装、使用、免责声明 |
| `AGENTS.md` | 是 | AI agent、维护者 | 协作规则、模块索引、信息优先级、长期约束 |
| `docs/` | 是 | 人和 AI | 架构、输入流程、词库、TUI、开发约束 |
| `memory/` | 否 | 本机 AI agent | 用户偏好、历史决策、本机长期背景 |

## 信息优先级

当信息冲突时：

1. 用户当前请求。
2. 当前代码事实。
3. `AGENTS.md`。
4. `docs/`。
5. 本机 `memory/`。
6. `README.md`。

`memory/` 只能辅助理解上下文，不能覆盖代码事实、用户当前请求或仓库内公开文档。

## 维护规则

- README 不放 AI 协作导航或模块维护细节。
- AGENTS 保持入口性质，不承载长篇设计说明。
- docs 按专题拆分，新增大功能优先新增或更新专题文档。
- memory 默认忽略，只保存本机长期记忆；需要共享的信息必须同步写入 AGENTS 或 docs。
- 文档和代码冲突时，先修代码或文档中的错误，再继续扩展功能。

## 常见更新场景

| 改动 | 需要更新 |
| --- | --- |
| 新增模块 | `docs/architecture.md`, `AGENTS.md` 模块约定 |
| 新增配置项 | `docs/tui.md`, `docs/typing-pipeline.md` |
| 修改输入节奏 | `docs/typing-pipeline.md`, 对应测试 |
| 修改词库格式 | `docs/dictionaries.md` |
| 新增长期协作规则 | `AGENTS.md` |
| 新增本机偏好或历史背景 | `memory/`，必要时同步公开文档 |
