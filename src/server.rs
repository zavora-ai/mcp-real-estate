use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};

use crate::attom::Attom;
use crate::bayut::Bayut;
use crate::nominatim::Nominatim;
use crate::uk_land_registry::UkLandRegistry;
use crate::world_bank::WorldBank;

// --- Input types ---

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UkTransactionSearch {
    /// Town name (e.g. "LONDON", "MANCHESTER")
    pub town: Option<String>,
    /// Postcode (e.g. "SW1A 2AA")
    pub postcode: Option<String>,
    /// Minimum price filter
    pub min_price: Option<u64>,
    /// Maximum price filter
    pub max_price: Option<u64>,
    /// Max results (default 10)
    pub limit: Option<u32>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TownInput {
    /// UK town name
    pub town: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GeoSearchInput {
    /// Search query (e.g. "apartments Westlands Nairobi")
    pub query: String,
    /// Max results (default 5)
    pub limit: Option<u32>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ReverseGeocodeInput {
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PropertySearchInput {
    /// Location name (e.g. "Dubai Marina", "Nairobi Westlands")
    pub location: String,
    /// Property type (e.g. "apartment", "villa", "office")
    pub property_type: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct MarketIndicatorInput {
    /// ISO country code (e.g. "KEN", "USA", "GBR", "ARE", "NGA")
    pub country_code: String,
    /// Year (default 2023)
    pub year: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CompareMarketsInput {
    /// ISO country codes to compare (e.g. ["USA", "GBR", "KEN", "ARE"])
    pub countries: Vec<String>,
    /// Year (default 2023)
    pub year: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UsPropertySearch {
    /// Street address
    pub address: Option<String>,
    /// ZIP code
    pub zipcode: Option<String>,
    /// City name
    pub city: Option<String>,
    /// Max results (default 5)
    pub limit: Option<u32>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AddressInput {
    /// Full street address
    pub address: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BayutSearchInput {
    /// Location ID or area name
    pub location: String,
    /// "for-sale" or "for-rent"
    pub purpose: Option<String>,
    /// "residential" or "commercial"
    pub category: Option<String>,
    /// Minimum price
    pub price_min: Option<u64>,
    /// Maximum price
    pub price_max: Option<u64>,
    /// Max results (default 5)
    pub limit: Option<u32>,
}

#[derive(Clone)]
pub struct RealEstateServer {
    pub uk_land_registry: UkLandRegistry,
    pub nominatim: Nominatim,
    pub world_bank: WorldBank,
    pub attom: Option<Attom>,
    pub bayut: Option<Bayut>,
}

#[tool_router]
impl RealEstateServer {
    // --- UK Land Registry ---

    #[tool(description = "Search UK property transactions (sales) by town, postcode, or price range")]
    async fn uk_search_transactions(&self, Parameters(input): Parameters<UkTransactionSearch>) -> String {
        let limit = input.limit.unwrap_or(10);
        match self.uk_land_registry.search_transactions(
            input.town.as_deref(), input.postcode.as_deref(),
            input.min_price, input.max_price, limit
        ).await {
            Ok(txns) => serde_json::to_string_pretty(&txns).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get average property price for a UK town based on recent transactions")]
    async fn uk_average_price(&self, Parameters(input): Parameters<TownInput>) -> String {
        match self.uk_land_registry.get_average_price(&input.town).await {
            Ok(Some(avg)) => serde_json::to_string_pretty(&serde_json::json!({
                "town": input.town, "average_price_gbp": avg, "currency": "GBP"
            })).unwrap_or_default(),
            Ok(None) => format!("No transactions found for {}", input.town),
            Err(e) => format!("Error: {e}"),
        }
    }

    // --- Geocoding / Location ---

    #[tool(description = "Search for locations, neighborhoods, or properties by name (global)")]
    async fn geocode_search(&self, Parameters(input): Parameters<GeoSearchInput>) -> String {
        let limit = input.limit.unwrap_or(5);
        match self.nominatim.search(&input.query, limit).await {
            Ok(results) => serde_json::to_string_pretty(&results).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Reverse geocode coordinates to get address and neighborhood info")]
    async fn reverse_geocode(&self, Parameters(input): Parameters<ReverseGeocodeInput>) -> String {
        match self.nominatim.reverse_geocode(input.lat, input.lon).await {
            Ok(Some(loc)) => serde_json::to_string_pretty(&loc).unwrap_or_default(),
            Ok(None) => "No location found for these coordinates".into(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Search for real estate agencies and properties in a location")]
    async fn search_properties_nearby(&self, Parameters(input): Parameters<PropertySearchInput>) -> String {
        match self.nominatim.search_properties(&input.location, input.property_type.as_deref()).await {
            Ok(results) => serde_json::to_string_pretty(&results).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    // --- World Bank Market Data ---

    #[tool(description = "Get real estate market indicators for a country (lending rates, inflation, GDP, urbanization)")]
    async fn get_market_indicators(&self, Parameters(input): Parameters<MarketIndicatorInput>) -> String {
        match self.world_bank.get_market_indicators(&input.country_code, input.year.as_deref()).await {
            Ok(indicators) => serde_json::to_string_pretty(&indicators).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Compare real estate market conditions across multiple countries")]
    async fn compare_markets(&self, Parameters(input): Parameters<CompareMarketsInput>) -> String {
        let codes: Vec<&str> = input.countries.iter().map(|s| s.as_str()).collect();
        match self.world_bank.compare_markets(&codes, input.year.as_deref()).await {
            Ok(indicators) => serde_json::to_string_pretty(&indicators).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    // --- US (ATTOM) ---

    #[tool(description = "Search US properties by address, ZIP code, or city (requires ATTOM API key)")]
    async fn us_search_properties(&self, Parameters(input): Parameters<UsPropertySearch>) -> String {
        match &self.attom {
            Some(attom) => {
                let limit = input.limit.unwrap_or(5);
                match attom.search_properties(input.address.as_deref(), input.zipcode.as_deref(), input.city.as_deref(), limit).await {
                    Ok(props) => serde_json::to_string_pretty(&props).unwrap_or_default(),
                    Err(e) => format!("Error: {e}"),
                }
            }
            None => "ATTOM backend not configured. Set ATTOM_API_KEY environment variable.".into(),
        }
    }

    #[tool(description = "Get US property valuation (AVM) by address (requires ATTOM API key)")]
    async fn us_get_valuation(&self, Parameters(input): Parameters<AddressInput>) -> String {
        match &self.attom {
            Some(attom) => match attom.get_valuation(&input.address).await {
                Ok(Some(val)) => serde_json::to_string_pretty(&val).unwrap_or_default(),
                Ok(None) => format!("No valuation found for {}", input.address),
                Err(e) => format!("Error: {e}"),
            },
            None => "ATTOM backend not configured. Set ATTOM_API_KEY environment variable.".into(),
        }
    }

    #[tool(description = "Get US property sales history by address (requires ATTOM API key)")]
    async fn us_sales_history(&self, Parameters(input): Parameters<AddressInput>) -> String {
        match &self.attom {
            Some(attom) => match attom.get_sales_history(&input.address).await {
                Ok(history) => serde_json::to_string_pretty(&history).unwrap_or_default(),
                Err(e) => format!("Error: {e}"),
            },
            None => "ATTOM backend not configured. Set ATTOM_API_KEY environment variable.".into(),
        }
    }

    // --- Middle East (Bayut) ---

    #[tool(description = "Search property listings in UAE/Middle East (requires Bayut/RapidAPI key)")]
    async fn me_search_listings(&self, Parameters(input): Parameters<BayutSearchInput>) -> String {
        match &self.bayut {
            Some(bayut) => {
                let limit = input.limit.unwrap_or(5);
                match bayut.search_listings(&input.location, input.purpose.as_deref(), input.category.as_deref(), input.price_min, input.price_max, limit).await {
                    Ok(listings) => serde_json::to_string_pretty(&listings).unwrap_or_default(),
                    Err(e) => format!("Error: {e}"),
                }
            }
            None => "Bayut backend not configured. Set BAYUT_API_KEY environment variable.".into(),
        }
    }
}

adk_mcp_sdk::mcp_2026_server! {
    server: RealEstateServer,
    task_tools: [],
    approval_tools: [],
    cache_ttl_ms: 60_000,
}
