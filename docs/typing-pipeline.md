# 输入流程、错字模拟与延迟策略

本文说明文本如何变成键盘输入，错字模拟如何插入输入流程，以及当前类人输入节奏是怎么计算的。

## 流程概览

```text
原始文本
  -> tokenize_with_config
  -> InputToken 列表
  -> delay_before
  -> 可选 typo plan
  -> type_token / typo wrong-backspace-retype / press_return
  -> delay_inside / backspace_delay / typo_retype_delay
  -> delay_after
```

对应文件：

- `src/typing/tokenizer.rs`
- `src/typing/typo.rs`
- `src/typing/dictionary_typo.rs`
- `src/typing/timing.rs`
- `src/typing/engine.rs`

## Token 类型

`TokenKind` 位于 `src/typing/token.rs`。

| 类型 | 含义 |
| --- | --- |
| `Word` | ASCII 单词、数字、下划线、连字符、英文撇号 |
| `CjkWord` | 中文或 CJK 词组 |
| `Whitespace` | 非换行空白 |
| `Punctuation` | 标点或其他单字符 |
| `Newline` | 换行 |

## 中文拆词

`tokenizer.rs` 会先识别连续 CJK 片段，再交给词库做最长匹配。

拆词规则：

- 开启中文拆词时，优先使用 Trie 前缀树做词库最长匹配。
- 词库没有命中时，使用 `fallback_cjk_len` 拆成 2 或 3 字左右的短片段。
- 关闭中文拆词时，每个 CJK 字符都是一个 `CjkWord`。

Trie 匹配位于 `src/typing/trie.rs`。它从当前位置逐字向前走前缀树，并记录最后一个命中的词尾，避免为每个候选窗口反复构造 `String`。

## 错字模拟

错字模拟目前只作用于 `CjkWord`。开启条件：

- `TypingConfig.typo_simulation` 为 `true`。
- `base_interval_ms >= 50`，避免在过快输入模式下硬塞错字。
- `typo_rate_percent` 通过稳定抖动决定是否命中，同一文本和位置的结果保持确定。

候选词来自当前加载的词库和内置词库。`dictionary_typo.rs` 会为每个词的非完整前缀建立索引，查询时从长度 `1` 到 `n - 1` 依次尝试，因此优先选择能产生候选的最短共同前缀。候选必须与正确词共享这个前缀，且不能是正确词本身；多个候选用稳定抖动确定一个结果。

`typo.rs` 会把候选词转换成输入计划：

```text
wrong_text -> backspaces -> retype_text
```

执行时，`engine.rs` 会先输入错误词，再退格删除错误后缀，最后重打正确后缀。当前错字来源是词库前缀相似，不基于拼音相似度或键盘物理位置。

## 中文词组延迟

中文词组总预算在 `timing.rs` 中计算：

```text
词组预算 = 字数 * 输入间隔 * 词长倍率 * 词频后置倍率
```

词长倍率：

| 词形 | 倍率 |
| --- | --- |
| 常用单字：你、我、他、她、它、的、地、得、了、等 | `0.9x` |
| 双字词 | `0.9x` |
| 三字词 | `1.0x` |
| 四字及以上词组 | `1.5x` |

词频后置倍率：

- 高频词最快只到 `1.0x`。
- 低频词会变慢，当前最慢约 `1.6x`。
- 词频倍率不会继续加速用户配置和词长倍率。

## 延迟分配

中文词组的延迟发生在词组前和词组内部：

```text
词前等待 + 词内字符间隔 = 词组预算
词后等待 = 0
```

如果开启“实验：关闭词组内间隔”：

```text
词前等待 = 词组预算
词内字符间隔 = 0
词后等待 = 0
```

这样表现为先想一下下一个词，然后快速输入整个词组。

## 非中文延迟

非中文 token 仍使用较轻的稳定抖动：

- ASCII 词：基础间隔加稳定抖动。
- 空白：较短间隔。
- 标点：较长停顿。
- 换行：最长停顿，并通过 Return 键输出。

## 修改建议

- 改中文预算公式：优先改 `cjk_word_delay_budget_ms`。
- 改词长倍率：优先改 `cjk_word_delay_scale_tenths`。
- 改词频影响：优先改 `src/typing/frequency.rs`。
- 改错字候选：优先改 `src/typing/dictionary_typo.rs`。
- 改错字计划或触发条件：优先改 `src/typing/typo.rs`。
- 改词前/词内/词后分配：改 `delay_before`、`delay_inside`、`delay_after`。
- 每次修改输入节奏，都需要更新 `src/typing/tests.rs`；修改错字模拟时同步更新 `src/typing/typo_tests.rs`。
