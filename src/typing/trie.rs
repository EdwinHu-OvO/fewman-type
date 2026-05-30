use std::collections::HashMap;

#[derive(Debug, Default)]
pub(crate) struct WordTrie {
    nodes: Vec<TrieNode>,
}

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, usize>,
    is_word: bool,
}

impl WordTrie {
    pub(crate) fn from_words<'a>(
        words: impl Iterator<Item = &'a String>,
        max_word_len: usize,
    ) -> Self {
        let mut trie = Self {
            nodes: vec![TrieNode::default()],
        };
        for word in words {
            trie.insert(word, max_word_len);
        }
        trie
    }

    pub(crate) fn longest_match(
        &self,
        chars: &[char],
        index: usize,
        max_len: usize,
    ) -> Option<usize> {
        let mut node_index = 0;
        let mut matched = None;

        for (offset, ch) in chars[index..].iter().copied().take(max_len).enumerate() {
            let Some(next_index) = self.nodes[node_index].children.get(&ch).copied() else {
                break;
            };

            node_index = next_index;
            if self.nodes[node_index].is_word {
                matched = Some(offset + 1);
            }
        }

        matched.filter(|length| *length >= 2)
    }

    fn insert(&mut self, word: &str, max_word_len: usize) {
        let char_count = word.chars().count();
        if !(2..=max_word_len).contains(&char_count) {
            return;
        }

        let mut node_index = 0;
        for ch in word.chars() {
            node_index = self.child_or_insert(node_index, ch);
        }
        self.nodes[node_index].is_word = true;
    }

    fn child_or_insert(&mut self, node_index: usize, ch: char) -> usize {
        if let Some(next_index) = self.nodes[node_index].children.get(&ch).copied() {
            return next_index;
        }

        let next_index = self.nodes.len();
        self.nodes.push(TrieNode::default());
        self.nodes[node_index].children.insert(ch, next_index);
        next_index
    }
}
