use carbond_lib::metrics::carbon_intensity::CarbonIntensity;
use carbond_lib::metrics::metric::{Metric, MetricError};
use uom::si::f64::{Energy, Mass, MassPerEnergy};

/// Loads the current carbon intensity from the file system.
pub async fn current_carbon_intensity() -> Result<MassPerEnergy, MetricError> {
    Ok(CarbonIntensity::try_read_from_fs().await?.get_value())
}

/// Loads the current carbon intensity from the file system and
/// calculates the carbon emission with a given energy.
pub async fn calculate_carbon_emission(energy: Energy) -> Result<Mass, MetricError> {
    let carbon_intensity: MassPerEnergy = current_carbon_intensity().await?;
    Ok(carbon_intensity * energy)
}
