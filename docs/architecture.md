# 架构说明

AutoTyper 当前分为三层：入口层、输入核心层、终端界面层。

## 总体结构

```text
src/
  main.rs
  typing/
  tui/
```

## 入口层

`src/main.rs` 负责：

- 启动 TUI 并接收 `InputSession`。
- 打印当前配置和词库来源。
- 监听全局键盘事件。
- 拦截等待状态下的 `Ctrl+V`。
- 在输入过程中响应 `Esc` 中断。
- 调用 `typing::type_text` 执行实际自动输入。

`main.rs` 不应该放入拆词、词库解析、TUI 绘制或延迟策略。

## 输入核心层

`src/typing/` 负责把文本转换为输入动作。

| 文件 | 职责 |
| --- | --- |
| `config.rs` | 输入配置结构 |
| `token.rs` | `InputToken` 和 `TokenKind` |
| `char_class.rs` | 字符分类和稳定抖动 |
| `common_words.rs` | 项目专用内置词和内置高频词入口 |
| `tokenizer.rs` | 文本 token 化，中文拆词 |
| `input_plan.rs` | 将 token 转成输入动作，处理成对符号的光标移动计划 |
| `dictionary.rs` | 词典加载、最长匹配、词频查询 |
| `trie.rs` | 前缀树索引，用于中文词组最长匹配 |
| `dictionary_typo.rs` | 基于词库前缀构造错字候选 |
| `typo.rs` | 生成中文错字输入、退格和重打计划 |
| `word_files.rs` | 查找可执行文件同目录下的 `*_words.yaml` |
| `yaml_words.rs` | 解析旧格式和带词频格式 YAML |
| `frequency.rs` | 根据词频计算后置速度倍率 |
| `timing.rs` | 根据 token、词长、词频和配置计算延迟 |
| `engine.rs` | 调用 `enigo` 输出键盘事件 |
| `tests.rs` | typing 层单元测试 |

内置高频词数据位于 `data/jieba_builtin_words.tsv`，通过 `include_str!` 编进程序。

输入核心层对外只暴露：

- `TypingConfig`
- `dictionary_sources`
- `type_text`

如果需要暴露更多接口，先确认调用方是否真的需要，避免把内部结构泄漏到外层。

## 终端界面层

`src/tui/` 负责输入页、配置页和主题。

| 文件 | 职责 |
| --- | --- |
| `mod.rs` | TUI 入口、全局回调、会话收尾 |
| `state.rs` | `InputSession`、页面状态、共享输入状态 |
| `page.rs` | `InputPage` 数据结构和状态同步 |
| `header.rs` | 顶部 logo、配置预览、词库提醒 |
| `config_page.rs` | 配置页渲染和配置项事件 |
| `view.rs` | `cursive::View` 实现和布局 |
| `render.rs` | 渲染常量、颜色和打印工具 |
| `theme.rs` | 透明主题设置 |

TUI 层不应该知道拆词细节，也不应该直接修改输入节奏算法。

## 数据流

```text
TUI 输入文本和配置
  -> main.rs 等待 Ctrl+V
  -> typing::type_text
  -> tokenizer 拆成 token
  -> input_plan 生成 token 和光标移动输入动作
  -> 可选 typo 计划生成错误词、退格和重打内容
  -> timing 计算词前/词内/词后、退格和重打延迟
  -> engine 调用 enigo 输出
```

## 维护边界

- 新增输入节奏规则：改 `typing/timing.rs`，必要时改 `typing/frequency.rs`。
- 新增成对符号输入规则：改 `typing/input_plan.rs` 和 `typing/input_plan_tests.rs`，必要时改 `typing/engine.rs`。
- 新增错字模拟规则：改 `typing/typo.rs`、`typing/dictionary_typo.rs`，必要时改 `typing/engine.rs` 和 `typing/timing.rs`。
- 新增词库字段：改 `typing/yaml_words.rs` 和 `typing/dictionary.rs`。
- 新增 TUI 配置项：同时改 `typing/config.rs`、`tui/config_page.rs`、`tui/header.rs`。
- 新增触发热键：优先改 `main.rs`，不要把全局热键逻辑放进 TUI。
