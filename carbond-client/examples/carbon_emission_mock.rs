use carbond_client::power_supply;
use powercap::{mock::MockBuilder, PowerCap};
use tempdir::TempDir;
use uom::si::{
    energy::{kilowatt_hour, millijoule},
    f64::{Energy, Mass, MassPerEnergy as CarbonIntensity},
    mass::gram,
    mass_per_energy::gram_per_kilowatt_hour,
};

use color_eyre::eyre::Result;

/// This examples shows how the power_supply crate can be used to calculate a carbon emission.
/// For that it takes a measured energy consumption and multiplies it with the current carbon
/// intensity to calculate a carbon emission.
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // record accumulated energy before and after function
    let accumulated_energy_before: Energy = simulate_rapl(0);
    demanding_function();
    let accumulated_energy_after: Energy = simulate_rapl(460);

    // calculate energy "usage" by subtracting the accumulated energy before the call from the accumulated energy after the call
    let energy: Energy = accumulated_energy_after - accumulated_energy_before;

    // load current carbon intensity
    let carbon_intensity: CarbonIntensity = power_supply::current_carbon_intensity().await?;

    // calculate the emission by multiplying the carbon intensity with the accumulated energy difference
    let carbon_emission: Mass = carbon_intensity * energy;
    println!(
        "Using an energy of {:?} mJ ({:?} kWh) while emitting a carbon intensity of {:?} gCO2/kWh results in a carbon emission of {:?} g.",
        energy.get::<millijoule>(),
        energy.get::<kilowatt_hour>(),
        carbon_intensity.get::<gram_per_kilowatt_hour>(),
        carbon_emission.get::<gram>()
    );
    Ok(())
}

#[allow(unused_must_use)]
fn demanding_function() {
    for x in 0u64..500 {
        x.pow(5u32);
    }
}

/// simulates the reading of intel-rapl total energy value
fn simulate_rapl(offset: u64) -> Energy {
    let root = TempDir::new("rapl").unwrap();
    MockBuilder::default()
        .with_enabled(true)
        .with_sockets(1)
        .with_socket_energy_generator(Box::new(move |_| 100 + offset))
        .with_socket_max_energy_range_generator(Box::new(|_| 500))
        .with_domain_energy_generator(Box::new(|_, _| 10))
        .with_domain_max_energy_range_generator(Box::new(|_, _| 50))
        .build(root.path())
        .unwrap();
    let cap = PowerCap::try_from(root.path()).unwrap();
    let value = cap.intel_rapl.total_energy().unwrap();
    Energy::new::<millijoule>(value as f64)
}
