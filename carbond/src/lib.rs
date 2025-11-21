use std::sync::Arc;

use carbond_lib::metrics::{
    carbon_intensity::CarbonIntensity, cpu_cycles::CpuCycleIntensity, metric::Metric,
};
use data::{
    config::internal::{Config, CpuConfig},
    state::State,
};
use errors::CarbondError;
use log::debug;

use tokio::sync::Mutex;
use uom::si::f64::{Mass, MassPerEnergy};

mod api;
mod config;
mod data;
pub mod errors;
mod fs;
pub mod scheduler;

/// Loads the configuration.
/// - Validates the file structure.
/// - Loads the config from the file system.
pub async fn load_config() -> Result<Config, CarbondError> {
    fs::validate_file_structure().await?;
    let config = fs::load_config().await?;
    let config = Config::try_parse(&config)?;
    Ok(config)
}

/// Loads the current state.
/// Tries to read from fs or uses default values.
pub async fn load_state() -> State {
    data::state::State::new().await
}

/// Updates the file system's stored carbon intensity.
/// - Downloads actual carbon intensity from WattTime.
/// - Writes carbon intensity to the file system.
pub async fn update_carbon_intensity(
    config: Arc<Mutex<Config>>,
    state: Arc<Mutex<State>>,
) -> Result<(), CarbondError> {
    let config = config.lock().await;
    debug!("Running PowerIntensityUpdate,");
    let carbon_intensity: CarbonIntensity = download_carbon_intensity(&config).await?;
    carbon_intensity.try_write_to_fs().await?;
    let mut state = state.lock().await;
    state.moer = carbon_intensity;
    Ok(())
}

/// Updates the fs's stored cpu intensity.
/// - Writes the emission per cycle of cpus to the file system.
pub async fn update_cpu_intensity(config: &Config) -> Result<(), CarbondError> {
    debug!("Running CpuIntensityUpdate,");
    let cpu_config = match config.device_config.as_ref().and_then(|dc| dc.cpu.as_ref()) {
        Some(cfg) => cfg,
        None => return Ok(()),
    };

    let cpu_intensity = load_cpu_intensity(cpu_config);
    cpu_intensity.try_write_to_fs().await?;

    Ok(())
}

fn load_cpu_intensity(cpu_config: &CpuConfig) -> CpuCycleIntensity {
    let mass_per_cycle: Mass = cpu_config.embodied_g / cpu_config.lifetime_cycles as f64;
    CpuCycleIntensity::from_value(mass_per_cycle)
}

async fn download_carbon_intensity(config: &Config) -> Result<CarbonIntensity, CarbondError> {
    let watt_time_config = config.watt_time.as_ref().ok_or(CarbondError::Config(
        errors::ConfigError::ConfigMissing(String::from("A config for watt_time is missing")),
    ))?;
    let api = api::Api::new(
        &watt_time_config.username,
        &watt_time_config.password,
        &watt_time_config.region,
    );
    let api = api.login().await?;
    let intensity: MassPerEnergy = api.get_watt_time_moer().await?;
    let instance = CarbonIntensity::from_value(intensity);
    Ok(instance)
}
