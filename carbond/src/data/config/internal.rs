use std::time::Duration;

use uom::si::f64::Mass;

#[derive(Debug, Clone)]
pub struct Config {
    pub logging_verbosity: usize,
    pub update_interval: Duration,
    pub electricity_map: Option<ElectricityMap>,
    pub watt_time: Option<WattTime>,
    pub device_config: Option<DeviceConfig>,
}

#[derive(Debug, Clone)]
pub struct ElectricityMap {
    pub region: String,
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct WattTime {
    pub region: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct DeviceConfig {
    pub cpu: Option<CpuConfig>,
}

#[derive(Debug, Clone)]
pub struct CpuConfig {
    pub embodied_g: Mass,
    pub lifetime_cycles: u64,
}
