use carbond_client::embodied;
use uom::si::{
    f64::Mass,
    mass::{microgram, picogram},
};

use color_eyre::eyre::Result;

/// This examples shows how the embodied emission module can be used to calculate emitted emobided carbon intensities of a cpu.
/// To calculate the CPU's carbon emission, cpu cylces are measured and multiplied with the embodied carbon intensity of the cpu per cycle.
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // record cpu cylce counter before and after function
    let pre;
    let after;
    unsafe {
        pre = core::arch::x86_64::_rdtsc();
        demanding_function();
        after = core::arch::x86_64::_rdtsc();
    }

    // calculate run CPU cycles by subtracting the value of the counter before the function from the value of the counter after the function.
    let cycles: u64 = after - pre;

    // load cpu's emobided intensity
    let cpu_embodied_emission: Mass = embodied::cpu::load_cpu_embodied_intensity().await?;

    // calculate the emission by multiplying the embodied cpu intensity with the number of cycles
    let carbon_emission: Mass = cpu_embodied_emission * cycles as f64;
    println!(
        "Running {:?} cycles while emitting a carbon intensity of {:?} pgCO2/cycle results in a carbon emission of {:?} mg.",
        cycles,
        cpu_embodied_emission.get::<picogram>(),
        carbon_emission.get::<microgram>(),
    );
    Ok(())
}

#[allow(unused_must_use)]
fn demanding_function() {
    for x in 0u64..500 {
        x.pow(5u32);
    }
}
