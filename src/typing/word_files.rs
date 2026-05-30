use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn dictionary_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut seen_files = HashSet::new();
    for dir in search_dirs() {
        collect_word_files(&dir, &mut seen_files, &mut files);
    }
    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    files
}

fn search_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(dir) = std::env::current_dir() {
        dirs.push(dir);
    }
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        dirs.push(dir.to_path_buf());
    }
    dedupe_dirs(dirs)
}

fn dedupe_dirs(dirs: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut seen = HashSet::new();
    dirs.into_iter()
        .filter_map(|dir| {
            let canonical = dir.canonicalize().unwrap_or(dir);
            seen.insert(canonical.clone()).then_some(canonical)
        })
        .collect()
}

fn collect_word_files(dir: &Path, seen: &mut HashSet<PathBuf>, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if is_word_yaml(&path) {
            let key = path.canonicalize().unwrap_or_else(|_| path.clone());
            if seen.insert(key) {
                files.push(path);
            }
        }
    }
}

fn is_word_yaml(path: &Path) -> bool {
    path.is_file()
        && path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with("_words.yaml"))
}
