# TUI 结构与交互

TUI 使用 `cursive` 的 crossterm 后端实现。入口在 `src/tui/mod.rs`。

## 页面

当前有两个页面：

- 输入页：输入或粘贴要自动输入的文本。
- 配置页：调整中文拆词、输入间隔、词组内间隔实验开关、错字模拟和错字率。

按 `F3` 在两个页面之间切换。

## 快捷键

| 按键 | 行为 |
| --- | --- |
| `F3` | 输入页和配置页切换 |
| `Ctrl+Enter` | 提交文本和配置 |
| `F2` | 提交文本和配置 |
| `Esc` | 退出 TUI；输入阶段则中断自动输入 |
| `↑` / `↓` | 配置页选择配置项 |
| `←` / `→` | 配置页调整输入间隔或错字率 |
| `Space` | 配置页切换开关 |

## 模块分工

| 文件 | 职责 |
| --- | --- |
| `mod.rs` | 创建 Cursive、绑定全局回调、输出 `InputSession` |
| `state.rs` | 页面状态和输入结果 |
| `page.rs` | `InputPage` 字段、状态同步、词库摘要 |
| `header.rs` | 顶部框、logo、配置预览、词库提醒 |
| `config_page.rs` | 配置页列表、配置项事件 |
| `view.rs` | `View` trait 实现、布局和页面切换 |
| `render.rs` | 高度常量、颜色、打印工具 |
| `theme.rs` | 透明主题 |

## 布局约束

- TUI 默认全屏铺满终端。
- 顶部区域高度固定为 `HEADER_HEIGHT`。
- 输入区域向下增高。
- 底部提示要始终保留空间。
- 输入框只使用极简上下框线。
- 不要重新引入 TextArea 的不透明背景。

相关代码：

- 高度常量：`src/tui/render.rs`
- 输入区高度计算：`src/tui/view.rs`
- 透明主题：`src/tui/theme.rs`

## 配置页同步

新增配置项时，需要同步修改：

- `src/typing/config.rs`：配置字段和默认值。
- `src/tui/config_page.rs`：配置页显示、选择、切换或调整。
- `src/tui/header.rs`：首页配置预览。
- `src/main.rs`：提交后的命令行确认信息。

错字模拟配置还会影响：

- `src/typing/typo.rs`：是否启用错字模拟、错字率命中和计划生成。
- `src/typing/dictionary_typo.rs`：基于词库前缀选择错字候选。

如果配置会影响输入节奏，还要更新：

- `src/typing/timing.rs`
- `src/typing/tests.rs`

## 样式约束

- 保持 Claude Code 风格的极简线框。
- 顶部 logo 和配置预览保留。
- 词库状态必须在首页显式显示。
- 控件文字要能在窄终端下被裁剪而不是破坏布局。
