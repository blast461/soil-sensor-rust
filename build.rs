fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Propagate ESP-IDF configuration to dependent crates and the linker
    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")?;
    Ok(())
}
