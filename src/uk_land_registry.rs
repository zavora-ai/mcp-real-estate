use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

use crate::types::PropertyTransaction;

pub struct UkLandRegistry {
    client: Client,
}

impl UkLandRegistry {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    pub async fn search_transactions(&self, town: Option<&str>, postcode: Option<&str>, min_price: Option<u64>, max_price: Option<u64>, limit: u32) -> Result<Vec<PropertyTransaction>> {
        let mut url = format!("http://landregistry.data.gov.uk/data/ppi/transaction-record.json?_pageSize={}", limit);
        if let Some(t) = town {
            url.push_str(&format!("&propertyAddress.town={}", t.to_uppercase()));
        }
        if let Some(p) = postcode {
            url.push_str(&format!("&propertyAddress.postcode={}", p.replace(' ', "+")));
        }
        if let Some(min) = min_price {
            url.push_str(&format!("&min-pricePaid={}", min));
        }
        if let Some(max) = max_price {
            url.push_str(&format!("&max-pricePaid={}", max));
        }
        let resp: Value = self.client.get(&url).send().await?.json().await?;
        let items = resp["result"]["items"].as_array().unwrap_or(&vec![]).clone();
        Ok(items.iter().map(|i| {
            let addr = &i["propertyAddress"];
            let ptype = i["propertyType"]["prefLabel"].as_array()
                .and_then(|a| a.first())
                .and_then(|v| v["_value"].as_str())
                .map(String::from);
            PropertyTransaction {
                source: "uk_land_registry".into(),
                region: "UK".into(),
                price: i["pricePaid"].as_f64().or_else(|| i["pricePaid"].as_str().and_then(|s| s.parse().ok())),
                currency: "GBP".into(),
                address: addr["street"].as_str().map(String::from),
                city: addr["town"].as_str().map(String::from),
                postcode: addr["postcode"].as_str().map(String::from),
                country: "United Kingdom".into(),
                property_type: ptype,
                new_build: i["newBuild"].as_bool(),
                date: i["hasTransaction"].as_array()
                    .and_then(|a| a.first())
                    .and_then(|t| t["transactionDate"].as_str())
                    .map(String::from),
            }
        }).collect())
    }

    pub async fn get_average_price(&self, town: &str) -> Result<Option<f64>> {
        let txns = self.search_transactions(Some(town), None, None, None, 50).await?;
        if txns.is_empty() { return Ok(None); }
        let total: f64 = txns.iter().filter_map(|t| t.price).sum();
        let count = txns.iter().filter(|t| t.price.is_some()).count();
        if count == 0 { return Ok(None); }
        Ok(Some(total / count as f64))
    }
}
