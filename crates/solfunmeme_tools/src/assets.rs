use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../.."] // Relative to crates/solfunmeme_tools/Cargo.toml
#[include = "README.md"]
pub struct ProjectAssets;
