<p align="center">
  <a href="README.md">English</a> · <a href="README.zh-CN.md">简体中文</a>
</p>

# Human-like Typing (FewmanType)

**“Far from human, but still trying hard to look like one.”**

**FewmanType** (pronounced similarly to “Human Type”; the name means “few humans,” hinting at how far it is from being human) is a Rust-powered project built for fun. It takes large blocks of text you paste and types them into the target window one character at a time, simulating the rhythm of a real person at a keyboard.

Whether you need to get around systems that detect pasted text or simply want to look impossibly fast in front of your friends, it may come in handy.

This project is also very far from human: it was built entirely on vibes, so please treat it as entertainment!

## ✨ Core Features

- **Global hotkey interception**: Copy your text, switch to the target window, and press `Ctrl+V`. The program intercepts the paste and immediately starts typing it for you.
- **Human-like timing**: Configure a base typing interval with random variation between keystrokes, avoiding perfectly mechanical output.
- **Three presets**: Switch between **Turbo**, **One-click Humanize**, and **Custom** presets, or fine-tune the settings yourself.
- **Smart Chinese word segmentation**: Load external dictionaries for CJK segmentation. Word groups are entered as units, with very short delays within a word and natural pauses between words.
- **Smart paired-symbol input**: Optionally handle matching pairs such as `()`, `<>`, and `（）` by typing the pair first, moving the cursor inside to enter the contents, and then moving back out.
- **Typo simulation and correction**: When enabled, the program uses the typo rate to find a Chinese dictionary word sharing the shortest prefix, types the “wrong” word, then backspaces and retypes the correct text like a person correcting a mistake.
- **Emergency stop at any time**: Press `ESC` whenever you need to stop the typing immediately.
- **Terminal user interface (TUI)**: A simple command-line interface for configuring parameters and entering the text to type.

## 🛠️ Installation and Usage

### Prerequisites

Make sure Rust is installed on your computer ([Rust website](https://www.rust-lang.org/)).

### Build and Run

1. Clone this repository:

   ```bash
   git clone https://github.com/EdwinHu-OvO/fewman-type-rs.git
   cd fewman-type-rs
   ```

2. Run the program:

   ```bash
   cargo run --release
   ```

   _(A `--release` build is recommended for more stable performance.)_

### How to Use

1. Launch the program and enter or paste the text you want it to type in the TUI.
2. Choose the **Turbo / One-click Humanize / Custom** preset, or adjust the **typing interval**, **Chinese word segmentation**, **symbol matching**, and **typo simulation / typo rate** settings manually. Then confirm.
3. Click the mouse where you want the text entered—for example, a chat box, Word document, code editor, or web page.
4. Press **`Ctrl+V`** and let the magic begin!
5. To stop partway through, press **`ESC`**.

## 📝 Configuration Files and Dictionaries

For stronger Chinese segmentation and typo simulation, place dictionary files such as `your_dict_words.yaml` in the same directory as the executable. At startup, the program automatically finds and loads dictionary files matching `*_words.yaml` in that directory. If none are found, it falls back to a small built-in word list.

When typo simulation is enabled, the program attempts to construct incorrect input for Chinese word groups according to the typo rate. Candidates come from other words in the same dictionary that share the shortest prefix; after typing the wrong word, it backspaces and retypes the correct suffix.

Two formats are currently supported.

Legacy format:

```yaml
words:
  - "你好"
  - "世界"
```

Format with frequencies:

```yaml
words:
  - text: "你好"
    frequency: 12345
  - text: "世界"
    frequency: 678
```

## ⚠️ Disclaimer

This project is for learning, research, and **fun** only. Do not use it to damage systems, cheat, or violate the rules of any platform. Because an overly realistic typing simulation may cause your friends, coworkers, or teachers to hold you in unnecessary awe, the author accepts no responsibility. It is a fun project—taking it seriously defeats the point.

## Third-party Data

The project includes 1,000 high-frequency Chinese words selected from `jieba/dict.txt` in `fxsjy/jieba`, stored in `data/jieba_builtin_words.tsv` in `word<TAB>frequency` format.

This data comes from the Jieba segmentation project, which is distributed under the MIT License:

- Project: https://github.com/fxsjy/jieba
- License file: https://github.com/fxsjy/jieba/blob/master/LICENSE
- Local license text: `licenses/jieba-MIT-LICENSE.txt`
- Original copyright notice: Copyright (c) 2013 Sun Junyi

---

_Built with Rust 🦀, `enigo`, `rdev`, and `cursive`._
