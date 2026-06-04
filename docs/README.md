# 文档总览

这组文档按模块化方式组织，目标是让人和 AI 都能快速定位上下文。

## 读者入口

- 想使用项目：先看根目录 [README.md](../README.md)。
- 想让 AI agent 接手维护：先看 [AGENTS.md](../AGENTS.md)，再看本目录。
- 想理解代码结构：看 [架构说明](architecture.md)。
- 想改中文拆词、错字模拟或输入节奏：看 [输入流程与延迟策略](typing-pipeline.md)。
- 想改词库格式、合并策略或雾凇词频：看 [词库格式与加载规则](dictionaries.md)。
- 想改终端界面：看 [TUI 结构与交互](tui.md)。
- 想理解文档和记忆如何分层：看 [文档架构](documentation-system.md)。
- 想继续开发：看 [开发与维护约束](development.md) 和 [AGENTS.md](../AGENTS.md)。

## AI 快速定位

| 任务 | 优先查看 | 主要文件 |
| --- | --- | --- |
| 修改输入配置 | `development.md` | `src/typing/config.rs`, `src/tui/config_page.rs` |
| 修改拆词逻辑 | `typing-pipeline.md` | `src/typing/tokenizer.rs`, `src/typing/dictionary.rs`, `src/typing/trie.rs` |
| 修改词频速度 | `typing-pipeline.md` | `src/typing/timing.rs`, `src/typing/frequency.rs` |
| 修改错字模拟 | `typing-pipeline.md` | `src/typing/typo.rs`, `src/typing/dictionary_typo.rs`, `src/typing/engine.rs` |
| 修改词库格式 | `dictionaries.md` | `src/typing/yaml_words.rs`, `src/typing/word_files.rs` |
| 修改 TUI 布局 | `tui.md` | `src/tui/header.rs`, `src/tui/view.rs`, `src/tui/config_page.rs` |
| 增加自动输入行为 | `architecture.md` | `src/typing/engine.rs`, `src/typing/timing.rs` |
| 调整文档体系 | `documentation-system.md` | `AGENTS.md`, `docs/` |

## 当前代码边界

- `src/main.rs` 负责程序生命周期、全局热键和启动输出。
- `src/typing/` 负责文本拆分、词库、错字模拟、延迟预算和键盘输出。
- `src/tui/` 负责终端界面、配置页和用户输入。
- 根目录 `rime_ice_words.yaml` 是本地裁剪词库，不是源码模块；程序只自动加载可执行文件同目录的 `*_words.yaml`。

## 文档维护原则

- 新增一个较大的功能时，优先新增专题文档或扩展对应专题，不要把所有信息塞进根 README。
- 文档要说明“行为是什么”和“代码在哪里”，方便人阅读，也方便 AI 检索。
- 文档中的模块名、文件名要与实际路径保持一致。
- `memory/` 是本机私有长期记忆，已被 `.gitignore` 忽略；公开项目事实必须沉淀到 `AGENTS.md` 或 `docs/`。

## 信息优先级

当信息冲突时，按以下顺序判断：

1. 用户当前请求。
2. 当前代码事实。
3. `AGENTS.md`。
4. `docs/`。
5. 本机 `memory/`。
6. `README.md`。

README 面向普通用户，可能不包含维护细节；不要用 README 覆盖代码事实或 `AGENTS.md` 中的协作约束。
