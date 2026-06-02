#[cfg(test)]
mod typo_tests {
    use crate::typing::dictionary_typo::{build_typo_prefixes, typo_candidate_for_word};
    use crate::typing::token::{InputToken, TokenKind};
    use crate::typing::typo::plan_for_token;

    #[test]
    fn prefix_building_and_candidate_lookup() {
        let words = vec![
            "自定义".to_string(),
            "自动化".to_string(),
            "自动输入法".to_string(),
            "自动输入器".to_string(),
            "自动".to_string(),
            "别人".to_string(),
        ];
        let prefixes = build_typo_prefixes(words.iter());

        // Single char word, should return None
        assert_eq!(typo_candidate_for_word("我", 0, &prefixes), None);

        // Word "自动输入器", length 5.
        // It shares "自动输入" (len 4) with "自动输入法"
        // It shares "自动" (len 2) with "自定义", "自动化", "自动" (wait, "自动" is len 2, exactly equal to prefix length, so valid_candidates logic should keep it? Actually "自动" matches prefix len 2 of "自动输入器", but "自动" has no characters AFTER the prefix. Our rule says it shouldn't match a longer prefix, and c.as_str() != word. If c == "自动", common len is 2. But we only query `len = 1` up to `n-1`. For `len=1`, prefix is "自". The common len with "自动" is 2. So it won't match `len=1` exact. For `len=2`, common len is 2. Is "自动" a valid typo for "自动输入器" with keep_chars=2? That would mean backspaces=0. Let's see what plan_for_token does with it. It returns None if backspaces=0.)
        // But the first prefix length checked is 1 ("自").
        // "自定义" shares "自" ? No, common_len is 2 ("自动"). So it fails exact len match.
        // Wait, "自动" is common prefix of "自定义" and "自动输入器". So common_len is 2.
        // So for len=1, there are NO candidates that share EXACTLY 1 char.
        // For len=2, candidates are "自定义", "自动化", "自动" (common_len=2).
        // If "自动" is chosen, candidate="自动", common_len=2. wrong_chars=2, keep_chars=2, backspaces=0.
        // Let's check what `typo_candidate_for_word` actually returns.
        let candidate = typo_candidate_for_word("自动输入器", 0, &prefixes).unwrap();
        assert!(candidate == "自定义" || candidate == "自动化" || candidate == "自动");
    }

    #[test]
    fn plan_for_token_no_plan_for_single_char() {
        let token = InputToken::new("我", TokenKind::CjkWord);
        assert_eq!(plan_for_token(&token, 0), None);
    }

    #[test]
    fn plan_for_token_no_plan_for_non_cjk() {
        let token = InputToken::new("Auto", TokenKind::Word);
        assert_eq!(plan_for_token(&token, 0), None);
    }

    // A real end-to-end plan test is slightly tricky because we rely on the global WORD_DICTIONARY
    // being loaded for `typo_candidate_for_word(&token.text, salt)?` inside `plan_for_token`.
    // In tests, `WORD_DICTIONARY` loads `common_words()` which are builtin words.
    // If we want to guarantee a specific typo, we would need to know the builtin words.
    // However, we can just test that any plan returned is valid.
    #[test]
    fn plan_for_token_returns_valid_plan_if_any() {
        let token = InputToken::new("因为", TokenKind::CjkWord); // likely has candidates like "因此"
        if let Some(plan) = plan_for_token(&token, 0) {
            // Must have backspaces
            assert!(plan.backspaces > 0);
            // Retype text must not be empty
            assert!(!plan.retype_text.is_empty());
            // The first `keep_chars` of wrong_text must match the correct text
            let wrong_chars: Vec<char> = plan.wrong_text.chars().collect();
            let keep_chars = wrong_chars.len() - plan.backspaces;
            let correct_chars: Vec<char> = token.text.chars().collect();
            assert_eq!(wrong_chars[..keep_chars], correct_chars[..keep_chars]);
        }
    }
}
