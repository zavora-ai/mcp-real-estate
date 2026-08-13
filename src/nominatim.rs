use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

use crate::types::{AddressDetail, LocationInfo};

#[derive(Clone)]
pub struct Nominatim {
    client: Client,
}

impl Nominatim {
    pub fn new() -> Self {
        Self { client: Client::builder().user_agent("mcp-real-estate/1.0").build().unwrap() }
    }

    pub async fn search(&self, query: &str, limit: u32) -> Result<Vec<LocationInfo>> {
        let url = format!(
            "https://nominatim.openstreetmap.org/search?q={}&format=json&limit={}&addressdetails=1",
            query.replace(' ', "+"), limit
        );
        let resp: Vec<Value> = self.client.get(&url).send().await?.json().await?;
        Ok(resp.iter().map(|r| parse_location(r)).collect())
    }

    pub async fn reverse_geocode(&self, lat: f64, lon: f64) -> Result<Option<LocationInfo>> {
        let url = format!(
            "https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=json&addressdetails=1&zoom=18",
            lat, lon
        );
        let resp: Value = self.client.get(&url).send().await?.json().await?;
        if resp["error"].is_string() { return Ok(None); }
        Ok(Some(parse_location(&resp)))
    }

    pub async fn search_properties(&self, location: &str, property_type: Option<&str>) -> Result<Vec<LocationInfo>> {
        let ptype = property_type.unwrap_or("real estate");
        let query = format!("{} {}", ptype, location);
        self.search(&query, 10).await
    }
}

fn parse_location(r: &Value) -> LocationInfo {
    let addr = &r["address"];
    LocationInfo {
        display_name: r["display_name"].as_str().unwrap_or_default().to_string(),
        lat: r["lat"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0.0),
        lon: r["lon"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0.0),
        location_type: r["type"].as_str().map(String::from),
        address: Some(AddressDetail {
            road: addr["road"].as_str().map(String::from),
            suburb: addr["suburb"].as_str().map(String::from),
            city: addr["city"].as_str().or(addr["town"].as_str()).map(String::from),
            state: addr["state"].as_str().map(String::from),
            postcode: addr["postcode"].as_str().map(String::from),
            country: addr["country"].as_str().map(String::from),
            country_code: addr["country_code"].as_str().map(String::from),
        }),
        bounding_box: r["boundingbox"].as_array().map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect()),
    }
}
