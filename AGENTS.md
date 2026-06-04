# AGENTS.md

## 项目定位

AutoTyper 是一个 Rust TUI 自动输入工具。核心目标是把用户输入的文本按更像真人的节奏输出，当前包含中文拆词、词频驱动延迟、词内间隔实验开关，以及本地 `*_words.yaml` 词库合并能力。

## 编码规范

- 坚持 KISS 原则：优先选择能解决当前问题的最简单实现，不为了“更完美”提前抽象、泛化或堆叠配置。
- 按高内聚、低耦合拆分模块；一个文件只负责一个清晰职责。
- 每个源码文件最好不超过 150 行。新增逻辑如果会让文件明显变长，优先拆到新模块。
- 保持外层接口小而稳定；跨模块只暴露调用方真正需要的函数和类型。
- 避免把 UI、词库解析、拆词、延迟策略和实际键盘输出混在同一文件。
- 默认保持现有行为不变，除非用户明确要求调整。
- 不做无关重构，不顺手改风格，不回滚用户已有改动。

## 模块约定

- `src/typing/config.rs`：输入配置。
- `src/typing/token.rs`：输入 token 类型。
- `src/typing/char_class.rs`：字符分类和稳定抖动。
- `src/typing/common_words.rs`：项目专用内置词和内置高频词入口。
- `src/typing/tokenizer.rs`：文本 token 化和中文拆词。
- `src/typing/input_plan.rs`：将 token 转成输入动作，处理成对符号的光标移动计划。
- `src/typing/dictionary.rs`：词典加载、词频范围和查询。
- `src/typing/trie.rs`：中文词组最长匹配前缀树。
- `src/typing/dictionary_typo.rs`：基于词库前缀的错字候选。
- `src/typing/typo.rs`：错字模拟计划和触发规则。
- `src/typing/word_files.rs`：查找可执行文件同目录下的 `*_words.yaml`。
- `src/typing/yaml_words.rs`：词库 YAML 解析。
- `src/typing/frequency.rs`：词频后置倍率计算。
- `src/typing/timing.rs`：输入延迟预算和分配。
- `src/typing/engine.rs`：调用 `enigo` 实际输入。
- `src/tui/*`：TUI 状态、渲染、配置页、主题和 View 实现。
- `data/jieba_builtin_words.tsv`：从 jieba 词库挑选的 1000 个内置高频词，通过 `include_str!` 编进程序。

## 文档指引

README 是面向人的项目介绍与使用说明，不要把 AI 协作导航、模块索引或维护细则塞进 README。

信息优先级：

1. 用户当前请求。
2. 当前代码事实。
3. `AGENTS.md`。
4. `docs/`。
5. 本机 `memory/`。
6. `README.md`。

如果文档和代码冲突，以代码事实为准，并在完成任务时提示需要更新文档。

面向人和 AI 的详细文档放在 `docs/`，按专题拆分：

- `docs/README.md`：文档总览和任务导航。
- `docs/architecture.md`：项目分层、模块职责、数据流。
- `docs/typing-pipeline.md`：拆词、token、输入延迟和词频节奏。
- `docs/dictionaries.md`：`*_words.yaml` 词库格式、加载和合并规则。
- `docs/tui.md`：TUI 页面、快捷键、布局和配置同步点。
- `docs/development.md`：开发约束、测试要求、常见修改落点。
- `docs/documentation-system.md`：README、AGENTS、docs、memory 的分层和优先级。

AI 接手任务时优先阅读本文件，然后按任务类型查看 `docs/` 对应专题。新增模块、配置项、词库格式或输入节奏时，同步更新对应专题文档。

## 长期记忆

`memory/` 用于保存本机长期有用的项目记忆。它不是任务日志，也不是临时草稿区。

`memory/` 已被 `.gitignore` 忽略，默认不进入仓库。不能把它当作项目公开事实来源；公开、可共享的长期约束应写进 `AGENTS.md` 或 `docs/`。

Agent 使用方式：

- 接手任务时，先读 `AGENTS.md`；如果任务涉及长期约束、用户偏好、历史决策或数据来源，且本机存在 `memory/`，再读 `memory/README.md` 和相关记忆文件。
- 当用户明确表达长期偏好、确认设计决策、说明数据来源或给出未来任务会反复用到的背景时，更新 `memory/`。
- 写入记忆时保持简短、可验证、长期有效；优先链接到代码或文档路径。
- 如果记忆被新决策替代，要更新或删除旧记忆，避免互相矛盾。
- 如果某条记忆应被所有协作者共享，必须同步沉淀到 `AGENTS.md` 或 `docs/`，不能只留在 `memory/`。

当前建议文件：

- `memory/project.md`：项目级长期约束和设计决策。
- `memory/user-preferences.md`：用户长期偏好。
- `memory/data-sources.md`：外部数据来源和本地化说明。

不要写入：

- 密钥、账号、Cookie、私人身份信息。
- 一次性任务进度或调试输出。
- 没有被用户确认的推测。
- 可以直接从当前代码读出的普通实现细节。

## 词库约束

- 词库文件必须使用 `*_words.yaml` 命名，程序只会自动合并可执行文件同目录下的多份词库。
- 程序会先加载 `src/typing/common_words.rs` 和 `data/jieba_builtin_words.tsv` 的内置词，再合并外部词库；没有外部词库时仅使用内置词。
- YAML 需要兼容旧格式和带词频格式：

```yaml
words:
  - "旧格式词条"
  - text: "带词频词条"
    frequency: 12345
```

- 不要把下载脚本、原始上游词库或临时转换产物长期放进项目；本项目只保留裁剪后的本地词库 YAML。

## 成对符号输入约束

- 成对符号计划位于 `src/typing/input_plan.rs`，不要把配对算法塞进 `engine.rs` 或 `tokenizer.rs`。
- 仅对 50 个原文字符内已匹配的非空成对符号移动光标；空对、未匹配或交叉错配时，当前符号按普通字符处理，后续 token 继续按当前规则规划。
- 孤儿闭符号按普通字符处理。
- 支持嵌套成对符号；修改配对规则时同步更新 `src/typing/input_plan_tests.rs`。

## 错字模拟约束

- 当前错字模拟只对 CJK token 生效，候选来自合并后的词库文本。
- 候选查找使用共享最短非完整前缀，不基于拼音相似度或键盘物理位置。
- 只有 `typo_simulation` 开启且 `base_interval_ms >= 50` 时启用错字模拟。
- 修改错字候选或计划时更新 `src/typing/typo_tests.rs`；如果改退格或重打延迟，也要更新 `src/typing/tests.rs`。

## 延迟策略约束

- 中文词组延迟由 `timing.rs` 统一控制。
- 词频倍率是后置倍率，最快只能到 `1.0x`，不能继续放大用户设定的加速偏移。
- 词组延迟发生在词组前和可选词内间隔中，词后不额外停顿。
- 修改输入节奏时必须补充或更新单元测试。

## TUI 约束

- TUI 默认撑满终端窗口。
- 输入区域保持极简上下框线，不要重新引入不透明输入背景。
- 底部提示必须始终保留高度，不能被输入框增长顶出窗口。
- 配置项新增时同步更新首页配置预览和配置页交互。

## 验证要求

改动 Rust 代码后至少运行：

```bash
cargo fmt
cargo test
cargo check
```

如果只改文档，可以不跑编译，但最终回复中要说明未运行代码验证。
