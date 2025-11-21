use carbond_lib::metrics::cpu_cycles::CpuCycleIntensity;
use carbond_lib::metrics::metric::{Metric, MetricError};
use uom::si::f64::Mass;

/// Loads the embodied intensity of the CPU from the file system.
pub async fn load_cpu_embodied_intensity() -> Result<Mass, MetricError> {
    Ok(CpuCycleIntensity::try_read_from_fs().await?.get_value())
}
