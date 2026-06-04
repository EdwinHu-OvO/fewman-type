# 开发与维护约束

本文面向继续开发本项目的人和 AI。

## 基本原则

- 高内聚、低耦合。
- 每个源码文件最好不超过 150 行。
- 先理解模块边界，再修改代码。
- 只改和任务直接相关的模块。
- 不因为顺手而重构无关文件。
- 不回滚用户已有改动。

更完整的代理规则见根目录 [AGENTS.md](../AGENTS.md)。

## 常用命令

```bash
cargo fmt
cargo test
cargo check
```

建议顺序：

1. 小步修改。
2. 运行 `cargo fmt`。
3. 运行 `cargo test`。
4. 运行 `cargo check`。

仅改文档时可以不运行编译验证。

## 新功能落点

| 功能类型 | 优先修改 |
| --- | --- |
| 新配置项或预设 | `typing/config.rs`, `tui/config_page.rs`, `tui/header.rs`, `main.rs` |
| 新输入节奏 | `typing/timing.rs`, `typing/tests.rs` |
| 新词频规则 | `typing/frequency.rs`, `typing/timing.rs` |
| 新成对符号输入规则 | `typing/input_plan.rs`, `typing/input_plan_tests.rs`, `typing/engine.rs` |
| 新错字模拟规则 | `typing/typo.rs`, `typing/dictionary_typo.rs`, `typing/engine.rs` |
| 新词库字段 | `typing/yaml_words.rs`, `typing/dictionary.rs` |
| 新拆词规则 | `typing/tokenizer.rs`, `typing/dictionary.rs`, `typing/trie.rs` |
| 新 TUI 区域 | `tui/header.rs`, `tui/view.rs`, `tui/render.rs` |
| 新全局热键 | `main.rs` |

## 测试要求

已经存在的 typing 层测试覆盖：

- Windows 换行归一。
- 常用中文词拆分。
- ASCII 单词合并。
- 关闭中文拆词。
- 中文词组延迟预算。
- 关闭词内间隔。
- 词长倍率。
- YAML 词频解析。
- 词频倍率不会快于 `1.0x`。
- 成对符号输入计划、可配置开关、50 字匹配上限、嵌套匹配、未匹配回退、孤儿闭符号处理和原 token 索引保留。
- 错字模拟启用阈值和错字率命中规则。
- 基于最短前缀的错字候选查找。
- 错字输入计划的基本合法性。

修改这些行为时要同步更新测试。

## 文件大小约束

新增逻辑前先看当前文件行数：

```powershell
Get-ChildItem -Recurse -Path src -File |
  Select-Object FullName,@{Name='Lines';Expression={(Get-Content $_.FullName).Count}}
```

如果某个文件将超过 150 行，优先拆模块。

## 文档约束

- 根 README 只做入口和快速使用。
- 详细说明放在 `docs/` 对应专题。
- 新增模块时更新 [架构说明](architecture.md)。
- 新增配置或行为时更新对应专题文档。
- 文档分层和信息优先级见 [文档架构](documentation-system.md)。
- `memory/` 默认被忽略；需要共享的长期事实必须写入 `AGENTS.md` 或 `docs/`。

## 数据文件约束

- `rime_ice_words.yaml` 是运行数据，不是源码；默认加载位置是可执行文件同目录，不是项目根目录。
- 不要提交临时下载脚本。
- 不要提交上游完整词库目录。
- 保持 `*_words.yaml` 的旧格式兼容。
