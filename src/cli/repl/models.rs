use openclaudia::{config, providers};

/// Get static list of models for a provider (fallback when API unavailable)
pub fn get_available_models(provider: &str) -> Vec<&'static str> {
    providers::available_models_for_provider(provider)
}

/// Fetch models dynamically from provider API (for OpenAI-compatible providers like LM Studio)
pub async fn fetch_dynamic_models(
    provider_config: &config::ProviderConfig,
    adapter: &dyn providers::ProviderAdapter,
) -> Option<Vec<String>> {
    if !adapter.supports_model_listing() {
        return None;
    }

    match providers::fetch_models(
        &provider_config.base_url,
        provider_config.api_key.as_ref(),
        adapter,
    )
    .await
    {
        Ok(models) => {
            let model_ids: Vec<String> = models.into_iter().map(|m| m.id).collect();
            if model_ids.is_empty() {
                None
            } else {
                Some(model_ids)
            }
        }
        Err(e) => {
            tracing::debug!("Failed to fetch models from API: {}", e);
            None
        }
    }
}
