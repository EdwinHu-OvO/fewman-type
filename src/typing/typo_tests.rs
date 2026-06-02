#[cfg(test)]
mod typo_tests {
    use crate::typing::TypingConfig;
    use crate::typing::dictionary_typo::{build_typo_prefixes, typo_candidate_for_word};
    use crate::typing::token::{InputToken, TokenKind};
    use crate::typing::typo::{can_simulate_typo, plan_for_token, should_apply_typo};

    fn config(base_interval_ms: u64, typo_simulation: bool) -> TypingConfig {
        TypingConfig {
            cjk_segmentation: true,
            base_interval_ms,
            skip_word_inner_delay: false,
            typo_simulation,
            typo_rate_percent: 15,
        }
    }

    #[test]
    fn typo_simulation_requires_human_interval() {
        assert!(!can_simulate_typo(config(49, true)));
        assert!(can_simulate_typo(config(50, true)));
        assert!(!can_simulate_typo(config(50, false)));
    }

    #[test]
    fn typo_rate_zero_never_applies() {
        assert!(!should_apply_typo("因为", 0, 0));
        assert!(!should_apply_typo("因为", 99, 0));
    }

    #[test]
    fn typo_rate_hundred_always_applies() {
        assert!(should_apply_typo("因为", 0, 100));
        assert!(should_apply_typo("因为", 99, 100));
    }

    #[test]
    fn typo_rate_is_deterministic() {
        let first = should_apply_typo("因为", 42, 15);
        let second = should_apply_typo("因为", 42, 15);
        assert_eq!(first, second);
    }

    #[test]
    fn typo_rate_clamps_above_hundred() {
        assert!(should_apply_typo("因为", 42, 255));
    }

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

        assert_eq!(typo_candidate_for_word("我", 0, &prefixes), None);

        let candidate = typo_candidate_for_word("自动输入器", 0, &prefixes).unwrap();
        assert_eq!(candidate, "自定义");
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
