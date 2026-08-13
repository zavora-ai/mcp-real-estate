use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

use crate::types::MarketIndicator;

#[derive(Clone)]
pub struct WorldBank {
    client: Client,
}

/// Indicators relevant to real estate markets
const INDICATORS: &[(&str, &str, &str)] = &[
    ("FR.INR.LEND", "Lending interest rate", "%"),
    ("FP.CPI.TOTL.ZG", "Inflation (CPI)", "%"),
    ("SP.URB.TOTL.IN.ZS", "Urban population", "% of total"),
    ("NY.GDP.PCAP.CD", "GDP per capita", "USD"),
    ("SP.POP.GROW", "Population growth", "%"),
];

impl WorldBank {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    pub async fn get_market_indicators(&self, country_code: &str, year: Option<&str>) -> Result<Vec<MarketIndicator>> {
        let date = year.unwrap_or("2023");
        let mut results = Vec::new();
        for (code, name, unit) in INDICATORS {
            let url = format!(
                "https://api.worldbank.org/v2/country/{}/indicator/{}?format=json&per_page=1&date={}",
                country_code, code, date
            );
            if let Ok(resp) = self.client.get(&url).send().await {
                if let Ok(data) = resp.json::<Value>().await {
                    if let Some(arr) = data.as_array().and_then(|a| a.get(1)).and_then(|v| v.as_array()) {
                        for item in arr {
                            results.push(MarketIndicator {
                                country: item["country"]["value"].as_str().unwrap_or(country_code).to_string(),
                                indicator: name.to_string(),
                                year: item["date"].as_str().unwrap_or(date).to_string(),
                                value: item["value"].as_f64(),
                                unit: unit.to_string(),
                            });
                        }
                    }
                }
            }
        }
        Ok(results)
    }

    pub async fn compare_markets(&self, countries: &[&str], year: Option<&str>) -> Result<Vec<MarketIndicator>> {
        let codes = countries.join(";");
        let date = year.unwrap_or("2023");
        let mut results = Vec::new();
        for (code, name, unit) in INDICATORS {
            let url = format!(
                "https://api.worldbank.org/v2/country/{}/indicator/{}?format=json&per_page={}&date={}",
                codes, code, countries.len(), date
            );
            if let Ok(resp) = self.client.get(&url).send().await {
                if let Ok(data) = resp.json::<Value>().await {
                    if let Some(arr) = data.as_array().and_then(|a| a.get(1)).and_then(|v| v.as_array()) {
                        for item in arr {
                            results.push(MarketIndicator {
                                country: item["country"]["value"].as_str().unwrap_or_default().to_string(),
                                indicator: name.to_string(),
                                year: item["date"].as_str().unwrap_or(date).to_string(),
                                value: item["value"].as_f64(),
                                unit: unit.to_string(),
                            });
                        }
                    }
                }
            }
        }
        Ok(results)
    }
}
