use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

use crate::types::PropertyListing;

/// Bayut/PropertyFinder style API — Middle East & Africa listings
/// Supports: UAE, Saudi Arabia, Egypt, Kenya, Nigeria (via RapidAPI or direct)
#[derive(Clone)]
pub struct Bayut {
    client: Client,
    api_key: String,
}

impl Bayut {
    pub fn new(api_key: String) -> Self {
        Self { client: Client::new(), api_key }
    }

    pub async fn search_listings(&self, location: &str, purpose: Option<&str>, category: Option<&str>, price_min: Option<u64>, price_max: Option<u64>, limit: u32) -> Result<Vec<PropertyListing>> {
        let purpose_val = purpose.unwrap_or("for-sale");
        let category_val = category.unwrap_or("residential");
        let mut url = format!(
            "https://bayut.p.rapidapi.com/properties/list?locationExternalIDs={}&purpose={}&categoryExternalID={}&hitsPerPage={}",
            location, purpose_val, category_val, limit
        );
        if let Some(min) = price_min { url.push_str(&format!("&priceMin={}", min)); }
        if let Some(max) = price_max { url.push_str(&format!("&priceMax={}", max)); }

        let resp: Value = self.client.get(&url)
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "bayut.p.rapidapi.com")
            .send().await?
            .json().await?;

        let hits = resp["hits"].as_array().unwrap_or(&vec![]).clone();
        Ok(hits.iter().map(|h| {
            let geo = &h["geography"];
            PropertyListing {
                source: "bayut".into(),
                region: "ME".into(),
                title: h["title"].as_str().map(String::from),
                price: h["price"].as_f64(),
                currency: h["currency"].as_str().unwrap_or("AED").to_string(),
                bedrooms: h["rooms"].as_u64().map(|v| v as u32),
                bathrooms: h["baths"].as_u64().map(|v| v as u32),
                area_sqft: h["area"].as_f64(),
                property_type: h["category"].as_array()
                    .and_then(|a| a.last())
                    .and_then(|c| c["name"].as_str())
                    .map(String::from),
                address: h["location"].as_array()
                    .map(|a| a.iter().filter_map(|l| l["name"].as_str()).collect::<Vec<_>>().join(", ")),
                city: geo.get("city").and_then(|c| c["name"].as_str()).map(String::from),
                country: geo.get("country").and_then(|c| c["name"].as_str()).unwrap_or("UAE").to_string(),
                listing_type: Some(purpose_val.to_string()),
                url: h["externalURL"].as_str().map(String::from),
            }
        }).collect())
    }
}
