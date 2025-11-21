use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct ConfigRepr {
    pub logging_verbosity: Option<usize>,
    pub update_interval: String,
    pub intensity_service: IntensityServiceRepr,
    pub device: Option<DeviceConfigRepr>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct IntensityServiceRepr {
    pub electricity_map: Option<ElectricityMapRepr>,
    pub watt_time: Option<WattTimeRepr>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ElectricityMapRepr {
    pub region: String,
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct WattTimeRepr {
    pub region: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DeviceConfigRepr {
    pub cpu: Option<CpuConfigRepr>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct CpuConfigRepr {
    pub embodied_g: f64,
    pub lifetime_cycles: u64,
}
