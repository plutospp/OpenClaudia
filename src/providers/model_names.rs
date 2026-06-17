//! Canonical default models and shorthand aliases (version suffix omitted).
//!
//! Users can pass `-m glm` instead of `-m glm-5.2`; [`normalize_model_name`]
//! expands family shorthands to the current default from
//! [`DEFAULT_MODELS_BY_TARGET`].

/// Per-provider default model when none is configured (crosslink #802).
pub const DEFAULT_MODELS_BY_TARGET: &[(&str, &str)] = &[
    ("anthropic", "claude-opus-4-8"),
    ("openai", "gpt-5.5"),
    ("google", "gemini-3.1-pro-preview"),
    ("zai", "glm-5.2"),
    ("deepseek", "deepseek-v4-pro"),
    ("qwen", "qwen-3.7-plus"),
    ("kimi", "kimi-k2.7-code"),
    ("minimax", "minimax-m3"),
];

/// Fallback for OpenAI-compatible targets not listed above (`lmstudio`, etc.).
pub const DEFAULT_MODEL_FALLBACK: &str = "gpt-5.5";

/// Anthropic tier shorthands — unlike other providers (which use a family
/// prefix like `glm` → `glm-5.2`), Anthropic models resolve by tier name.
/// Bump these when the flagship opus/sonnet/haiku ids change.
const ANTHROPIC_TIER_ALIASES: &[(&str, &str)] = &[
    ("opus", "claude-opus-4-8"),
    ("sonnet", "claude-sonnet-4-6"),
    ("haiku", "claude-haiku-4-5-20251001"),
];

/// Shorthand without a version suffix → canonical model id.
///
/// Family names (`glm`, `kimi`, …), provider slugs (`zai`, `anthropic`, …),
/// and common adapter aliases (`zhipu` → `glm-5.2`) all resolve here.
const MODEL_ALIASES: &[(&str, &str)] = &[
    ("glm", "glm-5.2"),
    ("gemini", "gemini-3.1-pro-preview"),
    ("claude", "claude-opus-4-8"),
    ("claude-opus", "claude-opus-4-8"),
    ("gpt", "gpt-5.5"),
    ("deepseek", "deepseek-v4-pro"),
    ("qwen", "qwen-3.7-plus"),
    ("kimi", "kimi-k2.7-code"),
    ("minimax", "minimax-m3"),
    ("anthropic", "claude-opus-4-8"),
    ("openai", "gpt-5.5"),
    ("google", "gemini-3.1-pro-preview"),
    ("zai", "glm-5.2"),
    ("zhipu", "glm-5.2"),
    ("alibaba", "qwen-3.7-plus"),
];

/// Look up the canonical default model for a provider target.
#[must_use]
pub fn default_model_for_target(target: &str) -> &'static str {
    DEFAULT_MODELS_BY_TARGET
        .iter()
        .find_map(|(t, m)| (*t == target).then_some(*m))
        .unwrap_or(DEFAULT_MODEL_FALLBACK)
}

/// Expand a shorthand model name to its canonical id.
///
/// Examples: `glm` → `glm-5.2`, `kimi` → `kimi-k2.7-code`. Already-canonical
/// names and unknown strings are returned unchanged (aside from trimming).
#[must_use]
pub fn normalize_model_name(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let key = trimmed.to_ascii_lowercase();

    for (_, model) in DEFAULT_MODELS_BY_TARGET {
        if key == model.to_ascii_lowercase() {
            return (*model).to_string();
        }
    }

    for (alias, canonical) in ANTHROPIC_TIER_ALIASES {
        if key == *alias {
            return (*canonical).to_string();
        }
    }

    for (alias, canonical) in MODEL_ALIASES {
        if key == *alias {
            return (*canonical).to_string();
        }
    }

    for (target, model) in DEFAULT_MODELS_BY_TARGET {
        if key == *target {
            return (*model).to_string();
        }
    }

    trimmed.to_string()
}

/// Static model list for a provider (fallback when the API has no model listing).
#[must_use]
pub fn available_models_for_provider(provider: &str) -> Vec<&'static str> {
    match provider {
        "anthropic" => vec![
            "opus",
            "sonnet",
            "haiku",
            "claude-opus-4-8",
            "claude-sonnet-4-6",
            "claude-haiku-4-5-20251001",
            "claude-sonnet-4-5-20250929",
            "claude-opus-4-5-20251101",
            "claude-opus-4-1-20250805",
            "claude-sonnet-4-20250514",
            "claude-opus-4-20250514",
        ],
        "openai" => vec![
            "gpt",
            "gpt-5.5",
            "gpt-5.2-codex",
            "gpt-5",
            "gpt-5-mini",
            "gpt-5-nano",
            "gpt-4.1",
            "gpt-4.1-mini",
            "gpt-4.1-nano",
            "o3",
            "o4-mini",
            "gpt-4o",
            "gpt-4o-mini",
        ],
        "google" => vec![
            "gemini",
            "gemini-3.1-pro-preview",
            "gemini-3-flash-preview",
            "gemini-2.5-pro",
            "gemini-2.5-flash",
            "gemini-2.5-flash-lite",
        ],
        "zai" => vec![
            "glm",
            "glm-5.2",
            "glm-5",
            "glm-4.7",
            "glm-4.7-flash",
            "glm-4.6",
            "glm-4.5-flash",
        ],
        "deepseek" => vec!["deepseek", "deepseek-v4-pro", "deepseek-reasoner"],
        "kimi" => vec!["kimi", "kimi-k2.7-code"],
        "minimax" => vec!["minimax", "minimax-m3"],
        "qwen" => vec![
            "qwen",
            "qwen-3.7-plus",
            "qwen3-max",
            "qwen-plus",
            "qwen-turbo",
            "qwq-plus",
            "qwen3-coder-plus",
        ],
        _ => vec!["gpt-5.5"],
    }
}

/// Resolve the model name for a chat session.
///
/// Priority: explicit override → provider config model → per-target default.
/// The result is passed through [`normalize_model_name`].
#[must_use]
pub fn resolve_model_name(
    model_override: Option<String>,
    provider_model: Option<String>,
    target: &str,
) -> String {
    let raw = model_override
        .or(provider_model)
        .unwrap_or_else(|| default_model_for_target(target).to_string());
    normalize_model_name(&raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_expands_anthropic_tier_aliases() {
        assert_eq!(normalize_model_name("opus"), "claude-opus-4-8");
        assert_eq!(normalize_model_name("OPUS"), "claude-opus-4-8");
        assert_eq!(normalize_model_name("sonnet"), "claude-sonnet-4-6");
        assert_eq!(normalize_model_name("haiku"), "claude-haiku-4-5-20251001");
    }

    #[test]
    fn normalize_expands_versionless_family_aliases() {
        assert_eq!(normalize_model_name("glm"), "glm-5.2");
        assert_eq!(normalize_model_name("GLM"), "glm-5.2");
        assert_eq!(normalize_model_name("kimi"), "kimi-k2.7-code");
        assert_eq!(normalize_model_name("minimax"), "minimax-m3");
        assert_eq!(normalize_model_name("qwen"), "qwen-3.7-plus");
        assert_eq!(normalize_model_name("deepseek"), "deepseek-v4-pro");
        assert_eq!(normalize_model_name("gemini"), "gemini-3.1-pro-preview");
        assert_eq!(normalize_model_name("claude"), "claude-opus-4-8");
        assert_eq!(normalize_model_name("gpt"), "gpt-5.5");
    }

    #[test]
    fn normalize_expands_provider_slugs() {
        assert_eq!(normalize_model_name("zai"), "glm-5.2");
        assert_eq!(normalize_model_name("zhipu"), "glm-5.2");
        assert_eq!(normalize_model_name("anthropic"), "claude-opus-4-8");
    }

    #[test]
    fn normalize_leaves_canonical_and_unknown_unchanged() {
        assert_eq!(normalize_model_name("glm-5.2"), "glm-5.2");
        assert_eq!(normalize_model_name("claude-sonnet-4-6"), "claude-sonnet-4-6");
        assert_eq!(normalize_model_name("custom-finetune"), "custom-finetune");
    }

    #[test]
    fn resolve_model_normalizes_override() {
        assert_eq!(
            resolve_model_name(Some("glm".into()), None, "anthropic"),
            "glm-5.2"
        );
    }

    #[test]
    fn default_models_table_is_canonical_for_resolver() {
        for (target, expected_model) in DEFAULT_MODELS_BY_TARGET {
            let got = resolve_model_name(None, None, target);
            assert_eq!(
                got, *expected_model,
                "DEFAULT_MODELS_BY_TARGET entry for `{target}` must round-trip"
            );
            assert_eq!(
                default_model_for_target(target),
                *expected_model,
                "default_model_for_target must agree for `{target}`"
            );
        }
        assert_eq!(
            default_model_for_target("definitely-not-a-known-target"),
            DEFAULT_MODEL_FALLBACK
        );
    }

    #[test]
    fn default_models_table_keys_are_unique_and_non_empty() {
        use std::collections::HashSet;
        let mut seen: HashSet<&str> = HashSet::new();
        for (target, _) in DEFAULT_MODELS_BY_TARGET {
            assert!(!target.is_empty(), "target key must not be empty");
            assert!(
                seen.insert(target),
                "duplicate target key `{target}` in DEFAULT_MODELS_BY_TARGET"
            );
        }
    }
}
