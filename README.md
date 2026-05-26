# mcp-real-estate

[![Crates.io](https://img.shields.io/crates/v/mcp-real-estate.svg)](https://crates.io/crates/mcp-real-estate)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Global property intelligence MCP server — transactions, valuations, market indicators, geocoding, and listings across US, UK, Middle East, Africa, and Asia-Pacific. **11 tools** with free public APIs (UK Land Registry, Nominatim, World Bank) plus optional commercial backends (ATTOM for US, Bayut for Middle East).

## Backends

| Backend | Region | Free? | What it provides |
|---------|--------|:-----:|-----------------|
| **UK Land Registry** | UK | ✅ | Property transactions, prices, types, postcodes |
| **Nominatim/OSM** | Global | ✅ | Geocoding, reverse geocoding, property search |
| **World Bank** | Global | ✅ | Lending rates, inflation, GDP, urbanization |
| **ATTOM** | US | 🔑 | Property search, AVM valuations, sales history |
| **Bayut** | UAE/ME | 🔑 | Listings (sale/rent), residential/commercial |

## Quick Start

```bash
cargo install mcp-real-estate

# Free backends work immediately (UK, geocoding, market data)
mcp-real-estate

# With US property data
ATTOM_API_KEY=your_key mcp-real-estate

# With Middle East listings
BAYUT_API_KEY=your_rapidapi_key mcp-real-estate
```

## Tools (11)

### UK Land Registry (free)
| Tool | Description |
|------|-------------|
| `uk_search_transactions` | Search UK property sales by town, postcode, price range |
| `uk_average_price` | Average price for a UK town from recent transactions |

### Geocoding (free, global)
| Tool | Description |
|------|-------------|
| `geocode_search` | Search locations/neighborhoods by name worldwide |
| `reverse_geocode` | Coordinates → address, neighborhood, country |
| `search_properties_nearby` | Find real estate in a location |

### Market Intelligence (free, global)
| Tool | Description |
|------|-------------|
| `get_market_indicators` | Country lending rates, inflation, GDP, urbanization |
| `compare_markets` | Side-by-side comparison of multiple countries |

### US Properties (ATTOM API key)
| Tool | Description |
|------|-------------|
| `us_search_properties` | Search by address, ZIP, or city |
| `us_get_valuation` | Automated Valuation Model (AVM) estimate |
| `us_sales_history` | Historical sales for a property |

### Middle East (Bayut/RapidAPI key)
| Tool | Description |
|------|-------------|
| `me_search_listings` | UAE/ME listings (sale/rent, residential/commercial) |

## Configuration

```json
{
  "mcpServers": {
    "real-estate": {
      "command": "mcp-real-estate",
      "env": {
        "ATTOM_API_KEY": "your_attom_key",
        "BAYUT_API_KEY": "your_rapidapi_key"
      }
    }
  }
}
```

## Global Coverage

| Continent | Coverage | Source |
|-----------|----------|--------|
| **North America** | US property data, valuations, sales history | ATTOM |
| **Europe** | UK transactions, prices, property types | UK Land Registry |
| **Middle East** | UAE/Saudi/Egypt listings | Bayut |
| **Africa** | Market indicators, geocoding | World Bank + Nominatim |
| **Asia-Pacific** | Market indicators, geocoding | World Bank + Nominatim |
| **Latin America** | Market indicators, geocoding | World Bank + Nominatim |

## Roadmap

### v1.1 — More regional backends
- Domain.com.au (Australia)
- Rightmove/Zoopla API (UK listings)
- Kenya Land Registry connector
- Nigeria FIRS property data

### v2.0 — Intelligence layer
- Rental yield calculator
- Price trend analysis
- Investment scoring
- Comparable sales engine

## License

Apache-2.0
