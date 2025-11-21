use carbond_lib::metrics::carbon_intensity::CarbonIntensity;
use carbond_lib::metrics::metric::Metric;

/// State shared accross carbond jobs.
#[derive(Debug)]
pub struct State {
    pub moer: CarbonIntensity,
}

impl State {
    /// Creates a new instance of carbond state.
    pub async fn new() -> State {
        State {
            moer: CarbonIntensity::try_read_from_fs()
                .await
                .unwrap_or(CarbonIntensity::neutral()),
        }
    }
}
