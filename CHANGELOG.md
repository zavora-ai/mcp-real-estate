# Changelog

## [1.1.0] - 2026-08-13

### Changed
- Upgraded to rmcp 3.1.2 and raised the minimum supported Rust version to 1.94.1.
- Added MCP 2026-07-28 stateless request handling while retaining MCP 2025-11-25 initialization compatibility.

### Added
- Per-request identity and protocol metadata, on-demand discovery/cache hints, and the configured Tasks and sealed MRTR approval policies.

## [1.0.0] — 2026-05-26

### Added
- **UK Land Registry backend** — property transactions, price search by town/postcode, average price calculation
- **Nominatim/OSM backend** — global geocoding, reverse geocoding, property/location search
- **World Bank backend** — real estate market indicators (lending rates, inflation, GDP per capita, urbanization, population growth) for any country
- **ATTOM backend** — US property search, AVM valuations, sales history (requires API key)
- **Bayut backend** — UAE/Middle East property listings, sale/rent, residential/commercial (requires RapidAPI key)
- 11 tools total covering 6 continents
- Free public APIs work with zero configuration
- Registry-compatible `mcp-server.toml` manifest
