mod types;
mod uk_land_registry;
mod nominatim;
mod world_bank;
mod attom;
mod bayut;
mod server;

use rmcp::{ServiceExt, transport::stdio};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let manifest = adk_mcp_sdk::ServerManifest::from_file(std::path::Path::new("mcp-server.toml"))?;
    let errors = manifest.validate();
    if !errors.is_empty() {
        eprintln!("Manifest warnings:");
        for e in &errors { eprintln!("  - {e}"); }
    }

    let uk_land_registry = uk_land_registry::UkLandRegistry::new();
    let nominatim = nominatim::Nominatim::new();
    let world_bank = world_bank::WorldBank::new();

    let attom = std::env::var("ATTOM_API_KEY").ok().map(|k| {
        eprintln!("ATTOM: configured");
        attom::Attom::new(k)
    });

    let bayut = std::env::var("BAYUT_API_KEY").ok().map(|k| {
        eprintln!("Bayut: configured");
        bayut::Bayut::new(k)
    });

    let server = server::RealEstateServer { uk_land_registry, nominatim, world_bank, attom, bayut };
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
