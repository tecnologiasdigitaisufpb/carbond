use const_format::concatcp;

pub const DATA_PATH: &str = "/var/carbond";

mod metric_type {
    pub const OPERATIONAL: &str = "/operational";
    pub const EMBODIED: &str = "/embodied";
}

/// Path for storing carbon intensity
pub const INTENSITY_PATH: &str =
    concatcp!(DATA_PATH, metric_type::OPERATIONAL, "/carbon-intensity");

/// Path for storing CPU cycle intensity
pub const CPU_PATH: &str = concatcp!(DATA_PATH, metric_type::EMBODIED, "/cpu");
