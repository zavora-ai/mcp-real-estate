use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

use crate::types::{PropertyListing, PropertyTransaction, PropertyValuation};

/// ATTOM Data Solutions — US property data (requires API key)
pub struct Attom {
    client: Client,
    api_key: String,
}

impl Attom {
    pub fn new(api_key: String) -> Self {
        Self { client: Client::new(), api_key }
    }

    async fn get(&self, endpoint: &str, params: &str) -> Result<Value> {
        let url = format!("https://api.gateway.attomdata.com/propertyapi/v1.0.0/{}?{}", endpoint, params);
        let resp = self.client.get(&url)
            .header("apikey", &self.api_key)
            .header("Accept", "application/json")
            .send().await?
            .json::<Value>().await?;
        Ok(resp)
    }

    pub async fn search_properties(&self, address: Option<&str>, zipcode: Option<&str>, city: Option<&str>, limit: u32) -> Result<Vec<PropertyListing>> {
        let mut params = format!("pagesize={}", limit);
        if let Some(a) = address { params.push_str(&format!("&address1={}", a.replace(' ', "+"))); }
        if let Some(z) = zipcode { params.push_str(&format!("&postalcode={}", z)); }
        if let Some(c) = city { params.push_str(&format!("&city={}", c.replace(' ', "+"))); }
        let resp = self.get("property/address", &params).await?;
        let properties = resp["property"].as_array().unwrap_or(&vec![]).clone();
        Ok(properties.iter().map(|p| {
            let addr = &p["address"];
            let building = &p["building"];
            PropertyListing {
                source: "attom".into(),
                region: "US".into(),
                title: addr["oneLine"].as_str().map(String::from),
                price: p["assessment"]["assessed"]["assdTtlValue"].as_f64(),
                currency: "USD".into(),
                bedrooms: building["rooms"]["beds"].as_u64().map(|v| v as u32),
                bathrooms: building["rooms"]["bathsFull"].as_u64().map(|v| v as u32),
                area_sqft: building["size"]["livingSize"].as_f64(),
                property_type: building["summary"]["propType"].as_str().map(String::from),
                address: addr["oneLine"].as_str().map(String::from),
                city: addr["locality"].as_str().map(String::from),
                country: "United States".into(),
                listing_type: Some("assessed".into()),
                url: None,
            }
        }).collect())
    }

    pub async fn get_valuation(&self, address: &str) -> Result<Option<PropertyValuation>> {
        let params = format!("address1={}", address.replace(' ', "+"));
        let resp = self.get("attomavm/detail", &params).await?;
        let prop = resp["property"].as_array().and_then(|a| a.first());
        match prop {
            Some(p) => {
                let avm = &p["avm"];
                Ok(Some(PropertyValuation {
                    source: "attom_avm".into(),
                    address: Some(address.to_string()),
                    estimated_value: avm["amount"]["value"].as_f64(),
                    currency: "USD".into(),
                    confidence: avm["amount"]["scr"].as_str().map(String::from),
                    comparable_sales: vec![],
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn get_sales_history(&self, address: &str) -> Result<Vec<PropertyTransaction>> {
        let params = format!("address1={}", address.replace(' ', "+"));
        let resp = self.get("saleshistory/detail", &params).await?;
        let properties = resp["property"].as_array().unwrap_or(&vec![]).clone();
        Ok(properties.iter().flat_map(|p| {
            let addr_line = p["address"]["oneLine"].as_str().unwrap_or_default().to_string();
            let sales = p["saleHistory"].as_array().unwrap_or(&vec![]).clone();
            sales.iter().map(move |s| PropertyTransaction {
                source: "attom".into(),
                region: "US".into(),
                price: s["amount"]["saleAmt"].as_f64(),
                currency: "USD".into(),
                address: Some(addr_line.clone()),
                city: None,
                postcode: None,
                country: "United States".into(),
                property_type: None,
                new_build: None,
                date: s["amount"]["saleRecDate"].as_str().map(String::from),
            }).collect::<Vec<_>>()
        }).collect())
    }
}
