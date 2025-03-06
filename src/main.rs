use anyhow::Result;
use esp_idf_svc::{log::EspLogger, sys};
use log::info;

fn main() -> Result<()> {
    sys::link_patches();
    EspLogger::initialize_default();
    info!("HAL init!");

    Ok(())
}
