use carbond::{load_config, load_state, update_carbon_intensity};
use carbond::{scheduler::Scheduler, update_cpu_intensity};
use color_eyre::Result;
use log::*;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let init_config = load_config().await?;
    stderrlog::new()
        .verbosity(init_config.logging_verbosity)
        .timestamp(stderrlog::Timestamp::Second)
        .show_module_names(true)
        .init()?;

    info!("Starting service.");
    let state = load_state().await;
    debug!("Initial state: {:?}", state);

    // one shot jobs
    debug!("Executing oneshot jobs.");
    update_cpu_intensity(&init_config).await?;

    debug!("Scheduling recurrent tasks.");
    let mut scheduler = Scheduler::new(init_config.clone(), state);

    // Schedule PowerIntensityUpdate job
    scheduler.schedule_job(
        String::from("PowerIntensityUpdate"),
        init_config.update_interval,
        |cfg, state| async { update_carbon_intensity(cfg, state).await },
    );

    // wait until ctrl+c or error occurs
    scheduler.await_service_end().await?;

    info!("Shutting down initialized.");
    scheduler.shutdown().await;
    info!("Gracefully shut down successful.");
    Ok(())
}
