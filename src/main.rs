mod tui;
mod typing;

use enigo::Enigo;
use rdev::{Event, EventType, Key, grab};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

// 标记是否收到开始输入的信号
static START_TYPING: AtomicBool = AtomicBool::new(false);
// 标记是否收到退出的信号
static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);
// 记录 Ctrl 键是否被按下
static CTRL_PRESSED: AtomicBool = AtomicBool::new(false);

fn callback(event: Event) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::ControlLeft) | EventType::KeyPress(Key::ControlRight) => {
            CTRL_PRESSED.store(true, Ordering::SeqCst);
            Some(event)
        }
        EventType::KeyRelease(Key::ControlLeft) | EventType::KeyRelease(Key::ControlRight) => {
            CTRL_PRESSED.store(false, Ordering::SeqCst);
            Some(event)
        }
        EventType::KeyPress(Key::KeyV) => {
            if CTRL_PRESSED.load(Ordering::SeqCst) {
                // 当处于等待开始状态，且触发 Ctrl+V 时
                if !START_TYPING.load(Ordering::SeqCst) {
                    START_TYPING.store(true, Ordering::SeqCst);
                    // 拦截此事件！返回 None 则系统就不会收到粘贴事件了
                    return None;
                }
            }
            Some(event)
        }
        EventType::KeyPress(Key::Escape) => {
            SHOULD_EXIT.store(true, Ordering::SeqCst);
            Some(event)
        }
        _ => Some(event),
    }
}

fn main() {
    // 运行 TUI 界面并获取输入
    let session = match tui::get_input() {
        Some(session) => session,
        None => {
            println!("未输入任何内容，程序退出。");
            return;
        }
    };
    let final_text = session.text;

    println!("==================================================");
    println!("准备就绪！文本长度: {} 个字符", final_text.chars().count());
    println!(
        "预设: {} | 中文拆词: {} | 符号匹配: {} | 输入间隔: {} ms | 词内间隔: {} | 错字模拟: {} | 错字率: {}%",
        session.config.preset.label(),
        if session.config.cjk_segmentation {
            "开启"
        } else {
            "关闭"
        },
        if session.config.pair_matching {
            "开启"
        } else {
            "关闭"
        },
        session.config.base_interval_ms,
        if session.config.skip_word_inner_delay {
            "关闭"
        } else {
            "开启"
        },
        if session.config.typo_simulation {
            "开启"
        } else {
            "关闭"
        },
        session.config.typo_rate_percent
    );
    let dictionaries = typing::dictionary_sources();
    if dictionaries.is_empty() {
        println!("词库: 未发现 *_words.yaml，使用内置小词表。");
    } else {
        println!("词库: {}", dictionaries.join(", "));
    }
    println!("请将光标移动到需要输入的位置，按 Ctrl+V 开始自动输入。");
    println!("按 ESC 键可强制停止输入并退出程序。");
    println!("==================================================");

    // 启动后台线程拦截并监听键盘事件
    thread::spawn(|| {
        if let Err(error) = grab(callback) {
            println!("Error: {:?}", error);
        }
    });

    // 等待触发信号
    loop {
        if SHOULD_EXIT.load(Ordering::SeqCst) {
            println!("已取消操作，程序退出。");
            return;
        }

        if START_TYPING.load(Ordering::SeqCst) {
            println!("检测到 Ctrl+V，开始输入...");
            break;
        }

        thread::sleep(Duration::from_millis(50));
    }

    // 给用户一点时间松开 Ctrl 和 V 键，避免按键冲突
    thread::sleep(Duration::from_millis(150));

    let mut enigo = Enigo::new();
    if !typing::type_text(&mut enigo, &final_text, session.config, &SHOULD_EXIT) {
        println!("\n检测到 ESC 键，已中断输入！");
        return;
    }

    println!("结束输入！\n欢迎下次使用！");
    thread::sleep(Duration::from_millis(2000));
}
