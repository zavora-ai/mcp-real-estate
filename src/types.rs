use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyTransaction {
    pub source: String,
    pub region: String,
    pub price: Option<f64>,
    pub currency: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postcode: Option<String>,
    pub country: String,
    pub property_type: Option<String>,
    pub new_build: Option<bool>,
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyListing {
    pub source: String,
    pub region: String,
    pub title: Option<String>,
    pub price: Option<f64>,
    pub currency: String,
    pub bedrooms: Option<u32>,
    pub bathrooms: Option<u32>,
    pub area_sqft: Option<f64>,
    pub property_type: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: String,
    pub listing_type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationInfo {
    pub display_name: String,
    pub lat: f64,
    pub lon: f64,
    pub location_type: Option<String>,
    pub address: Option<AddressDetail>,
    pub bounding_box: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressDetail {
    pub road: Option<String>,
    pub suburb: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postcode: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketIndicator {
    pub country: String,
    pub indicator: String,
    pub year: String,
    pub value: Option<f64>,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyValuation {
    pub source: String,
    pub address: Option<String>,
    pub estimated_value: Option<f64>,
    pub currency: String,
    pub confidence: Option<String>,
    pub comparable_sales: Vec<PropertyTransaction>,
}
