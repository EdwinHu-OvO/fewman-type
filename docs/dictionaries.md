# 词库格式与加载规则

AutoTyper 使用本地 YAML 词库进行中文拆词，并可读取词频影响输入节奏。

## 文件发现

词库文件必须命名为：

```text
*_words.yaml
```

程序会搜索两个位置：

- 当前运行目录。
- 可执行文件所在目录。

多份词库会自动合并。文件发现逻辑位于 `src/typing/word_files.rs`。

## YAML 格式

当前兼容两种格式。

旧格式：

```yaml
words:
  - "自动输入"
  - "雨课堂"
```

带词频格式：

```yaml
words:
  - text: "自动输入"
    frequency: 12345
  - text: "雨课堂"
    frequency: 678
```

解析逻辑位于 `src/typing/yaml_words.rs`。

## 合并规则

合并逻辑位于 `src/typing/dictionary.rs`。

- 词条文本相同则视为同一个词。
- 如果多份词库都有词频，保留更高的 `frequency`。
- 没有词频的词仍可用于拆词，但不参与词频速度调整。
- 没有外部词库时，会使用 `src/typing/common_words.rs` 中的内置小词表。

## 雾凇词库

当前根目录的 `rime_ice_words.yaml` 是雾凇拼音词库裁剪后的本地版本，只保留拆词所需的 `text` 和 `frequency`。

本项目不保留：

- 上游完整仓库。
- 临时下载脚本。
- 原始 `.dict.yaml` 文件。
- 拼音编码列。

这符合“不要内置外挂，只保留本地词库数据”的约束。

## 词频用途

词频不会影响拆词命中顺序。拆词仍然是最长匹配优先。

词频只影响输入速度：

```text
最终中文词组预算 = 基础词长预算 * 词频后置倍率
```

词频越高越接近 `1.0x`，词频越低越慢。

## 修改建议

- 要支持新 YAML 字段：改 `yaml_words.rs`。
- 要改变多词库合并策略：改 `dictionary.rs` 的合并逻辑。
- 要改变词库搜索位置：改 `word_files.rs`。
- 要改变词频如何影响速度：改 `frequency.rs` 和 `timing.rs`。

## 注意事项

- `rime_ice_words.yaml` 很大，编辑时尽量使用脚本或流式工具。
- 不要把词库解析写成依赖固定行号或固定排序。
- 保持旧格式兼容，避免用户已有 `*_words.yaml` 失效。
