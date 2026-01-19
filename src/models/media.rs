//! Data models for media model responses.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Media model from the API (text-to-image, video, speech, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaModel {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub creator: MediaCreator,
    #[serde(default)]
    pub elo: Option<f64>,
    #[serde(default)]
    pub rank: Option<u32>,
    #[serde(default, rename = "categoryBreakdown")]
    pub category_breakdown: Option<HashMap<String, CategoryScore>>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Media model creator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCreator {
    pub name: String,
    pub slug: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Score for a category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryScore {
    pub elo: Option<f64>,
    pub rank: Option<u32>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

impl MediaModel {
    /// Get the ELO score or default.
    pub fn elo_score(&self) -> f64 {
        self.elo.unwrap_or(0.0)
    }

    /// Get the rank or default.
    pub fn ranking(&self) -> u32 {
        self.rank.unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_model_deserialization() {
        let json = r#"{
            "id": 1,
            "name": "DALL-E 3",
            "slug": "dall-e-3",
            "creator": {
                "name": "OpenAI",
                "slug": "openai"
            },
            "elo": 1250.5,
            "rank": 1,
            "categoryBreakdown": {
                "photorealistic": { "elo": 1300.0, "rank": 1 },
                "artistic": { "elo": 1200.0, "rank": 2 }
            }
        }"#;

        let model: MediaModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.id, 1);
        assert_eq!(model.name, "DALL-E 3");
        assert_eq!(model.elo_score(), 1250.5);
        assert_eq!(model.ranking(), 1);
        assert!(model.category_breakdown.is_some());

        let cats = model.category_breakdown.unwrap();
        assert_eq!(cats.get("photorealistic").unwrap().elo, Some(1300.0));
    }

    #[test]
    fn test_media_model_minimal() {
        let json = r#"{
            "id": 2,
            "name": "Midjourney",
            "slug": "midjourney",
            "creator": { "name": "Midjourney", "slug": "midjourney" }
        }"#;

        let model: MediaModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.id, 2);
        assert_eq!(model.elo_score(), 0.0);
        assert_eq!(model.ranking(), 0);
        assert!(model.category_breakdown.is_none());
    }
}
