use super::dictionary::WordEntry;

pub(crate) fn parse_yaml_word_entries(content: &str) -> Vec<(String, WordEntry)> {
    let mut entries = Vec::new();
    let mut pending: Option<(String, WordEntry)> = None;

    for line in content.lines() {
        let line = line.trim();
        let Some(item) = line.strip_prefix("- ") else {
            read_frequency(line, &mut pending);
            continue;
        };

        if let Some(entry) = pending.take() {
            entries.push(entry);
        }

        let item = item.trim();
        if let Some(text) = item.strip_prefix("text:") {
            if let Some(word) = parse_yaml_scalar(text.trim()) {
                pending = Some((word, WordEntry { frequency: None }));
            }
        } else if let Some(word) = parse_yaml_scalar(item) {
            entries.push((word, WordEntry { frequency: None }));
        }
    }

    if let Some(entry) = pending {
        entries.push(entry);
    }

    entries
}

fn read_frequency(line: &str, pending: &mut Option<(String, WordEntry)>) {
    if let Some((_, entry)) = pending.as_mut()
        && let Some(frequency) = line.strip_prefix("frequency:")
    {
        entry.frequency = frequency.trim().parse().ok();
    }
}

fn parse_yaml_scalar(value: &str) -> Option<String> {
    let value = value.trim();
    let value = value
        .strip_prefix('"')
        .and_then(|word| word.strip_suffix('"'))
        .unwrap_or(value)
        .replace("\\\"", "\"")
        .replace("\\\\", "\\");

    (!value.is_empty()).then_some(value)
}
