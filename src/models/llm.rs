//! Data models for LLM responses.

use serde::{Deserialize, Serialize};

/// LLM Model from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmModel {
    pub id: u64,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub short_name: Option<String>,
    pub creator: ModelCreator,
    #[serde(default)]
    pub evaluations: Option<Evaluations>,
    #[serde(default)]
    pub pricing: Option<Pricing>,
    #[serde(default)]
    pub speed: Option<Speed>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Model creator information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCreator {
    pub name: String,
    pub slug: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Model evaluations/benchmarks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evaluations {
    #[serde(rename = "artificialAnalysisIntelligenceIndex")]
    pub intelligence_index: Option<f64>,
    #[serde(rename = "chatbotArenaElo")]
    pub chatbot_arena_elo: Option<f64>,
    #[serde(rename = "lmsysArenaElo")]
    pub lmsys_arena_elo: Option<f64>,
    #[serde(rename = "mmlu")]
    pub mmlu: Option<f64>,
    #[serde(rename = "humanEval")]
    pub human_eval: Option<f64>,
    #[serde(rename = "gpqa")]
    pub gpqa: Option<f64>,
    #[serde(rename = "math")]
    pub math: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Pricing information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pricing {
    #[serde(rename = "inputTokens")]
    pub input_tokens: Option<f64>,
    #[serde(rename = "outputTokens")]
    pub output_tokens: Option<f64>,
    #[serde(rename = "blendedTokens")]
    pub blended_tokens: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Speed metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Speed {
    #[serde(rename = "tokensPerSecond")]
    pub tokens_per_second: Option<f64>,
    #[serde(rename = "timeToFirstToken")]
    pub time_to_first_token: Option<f64>,
    #[serde(rename = "latency")]
    pub latency: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

impl LlmModel {
    /// Get the display name.
    pub fn display_name(&self) -> &str {
        self.short_name.as_deref().unwrap_or(&self.name)
    }

    /// Get intelligence index or default.
    pub fn intelligence(&self) -> Option<f64> {
        self.evaluations.as_ref()?.intelligence_index
    }

    /// Get input token price.
    pub fn input_price(&self) -> Option<f64> {
        self.pricing.as_ref()?.input_tokens
    }

    /// Get output token price.
    pub fn output_price(&self) -> Option<f64> {
        self.pricing.as_ref()?.output_tokens
    }

    /// Get tokens per second.
    pub fn tps(&self) -> Option<f64> {
        self.speed.as_ref()?.tokens_per_second
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_model_deserialization() {
        let json = r#"{
            "id": 123,
            "name": "GPT-4",
            "slug": "gpt-4",
            "short_name": "GPT-4",
            "creator": {
                "name": "OpenAI",
                "slug": "openai"
            },
            "evaluations": {
                "artificialAnalysisIntelligenceIndex": 85.5,
                "chatbotArenaElo": 1200
            },
            "pricing": {
                "inputTokens": 0.03,
                "outputTokens": 0.06
            },
            "speed": {
                "tokensPerSecond": 50.0
            }
        }"#;

        let model: LlmModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.id, 123);
        assert_eq!(model.name, "GPT-4");
        assert_eq!(model.slug, "gpt-4");
        assert_eq!(model.creator.name, "OpenAI");
        assert_eq!(model.intelligence(), Some(85.5));
        assert_eq!(model.input_price(), Some(0.03));
        assert_eq!(model.tps(), Some(50.0));
    }

    #[test]
    fn test_llm_model_with_missing_fields() {
        let json = r#"{
            "id": 456,
            "name": "Test Model",
            "slug": "test-model",
            "creator": {
                "name": "Test Corp",
                "slug": "test-corp"
            }
        }"#;

        let model: LlmModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.id, 456);
        assert!(model.evaluations.is_none());
        assert!(model.pricing.is_none());
        assert!(model.speed.is_none());
        assert_eq!(model.intelligence(), None);
    }

    #[test]
    fn test_display_name() {
        let json = r#"{
            "id": 1,
            "name": "Full Model Name",
            "slug": "model",
            "short_name": "Short",
            "creator": { "name": "Creator", "slug": "creator" }
        }"#;

        let model: LlmModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.display_name(), "Short");

        let json_no_short = r#"{
            "id": 2,
            "name": "Full Model Name",
            "slug": "model",
            "creator": { "name": "Creator", "slug": "creator" }
        }"#;

        let model_no_short: LlmModel = serde_json::from_str(json_no_short).unwrap();
        assert_eq!(model_no_short.display_name(), "Full Model Name");
    }
}
