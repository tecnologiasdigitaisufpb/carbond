<!-- 
SPDX-FileCopyrightText: 2023 Andreas Schmidt <andreas.schmidt@cs.uni-saarland.de>

SPDX-License-Identifier: MIT
-->

<img src="docs/carbond.svg" alt="carbond" />

an operating-system daemon for carbon awareness

<!--[![Crates.io](https://img.shields.io/crates/l/carbond.svg)](https://crates.io/crates/carbond)-->
<!--[![Crates.io](https://img.shields.io/crates/v/carbond.svg)](https://crates.io/crates/carbond)-->
<!--[![Documentation](https://docs.rs/carbond/badge.svg)](https://docs.rs/carbond)-->
[![Minimum Stable Rust Version](https://img.shields.io/badge/Rust-1.63.0%2B-orange.svg)](https://rustup.rs/)
<!--[![ClearlyDefined Score](https://img.shields.io/clearlydefined/score/crate/cratesio/-/carbond/0.0.0?label=ClearlyDefined%20Score)](https://clearlydefined.io/definitions/crate/cratesio/-/carbond/0.0.0)-->
<!--[![REUSE status](https://api.reuse.software/badge/gitlab.com/netzdoktor/carbond)](https://api.reuse.software/info/gitlab.com/netzdoktor/carbond)-->
<!--[![Dependency Status](https://deps.rs/repo/gitlab/netzdoktor/carbond/status.svg)](https://deps.rs/repo/gitlab/netzdoktor/carbond)-->

The goal of this project is to provide a system service and a matching interfacing libraries to make software carbon-aware.

**The end goal of this project is to enable tracking carbon footprint of anything from whole processes down to single programming language functions.**

## Overview

Carbond is an operating-system daemon for carbon awareness.
It comes with a Rust library for easy consumption of the daemon's file system API but can be used by any program with access to the file system.

At the moment, the project consists of the following crates:

* `carbond`: Executable Binary
  * Query carbon intensity.
  * Tracking battery carbon intensity.
  * Manage system resource embodied emissions (e.g. CPU).
  * Provide collected information via a file system API under `/var/carbond`.
* `carbond-client` Library
  * Type-safe interface to the managed information in `/var/carbond`.
  * Functionality to calculate carbon emissions from collected operational data.
* `carbond-lib` Library
  * Provide definitions of embodied and operational metrics.
  * Common funcitonality used by both the `carbond` binary and the `carbond-client` lib.
* `carbond-battery` Library
  * Functionality for battery tracking.
  * Exposes information about state of charge and stored carbon in the battery.
* `carbond-macros` Library
  * Provide macros to wrap arbitrary `fn` with carbon tracking.
  * Exposes information about run CPU cycles and used operational energy.

## `carbond` Service
This binary is the main component of `carbond`.
It provides carbon information to other system services and applications via its file system API and connects to external data providers to collect Marginal Operating Emissions Rate (MOER) from providers such as `WattTime`. The data is converted into standardized uom units and made accessible via the file system.

This binary is provided as a `.deb` package for easy installation of the daemon on compatible Linux systems.

### Config

The config for `carbond` is stored at `/etc/carbond/config.toml`.

```toml
update_interval = "1h" # Interval for updating MOER from the external data provider.

[intensity_service.watt_time]
region = "CAISO_NORTH"
username = "..."
password = "..."

# specify cpu data yourself
[device.cpu.0]
embodied_g = 1000
lifetime_cycles = 1000000000000000
```

### API
Carbond exposes its collected data via an file system based API under `/var/carbond`:
* Operational:
  * Intensity: gCO2/kWh
* Embodied:
  * CPUs: gCO2/cycle

### Testing

Can be tested with a provided Dockerfile which compiles `carbond` using the musl libc implementation.

To start the testing environment open a terminal on your host system:

```rust
docker build -t carbond .
docker run -dt carbond
docker cp *host_config_path* *container_id*:/etc/carbond/config.toml
docker exec -it *container_id* bash
```

Now, you should have access to a terminal inside the container with the created binary.

```sh
./carbond
cat /var/carbond/operational/carbon-intensity
```

You now should see the current carbon intensity that was received from the API.

## `carbond-client` Library

A library for the Rust programming language that allows easy use of the `carbond` file system API.

Provides functionality to calculate emitted emissions from collected runtime data, such as the emitted embodied emission of the CPU from the number of CPU cycles executed.

The runtime data can either be collected manually and used with the carbon intensities to calculate the emitted emissions or collected through provided macros.

### Example
```Rust
use carbond_macros::carbond_track;
use carbond_client::embodied;

#[carbond_track]
fn tracked_function() -> u32 {
    let a = 50;
    let b = 22;
    b + a
}

#[tokio::main]
fn main() {
    // run tracked function
    let track: CarbonTracking<u32> = tracked_function();
    println!("{:#?}", track);

    // load cpu's embodied intensity (per cycle)
    let cpu_embodied_emission: Mass = embodied::cpu::load_cpu_embodied_intensity().await?;

    // calculate the emission by multiplying the embodied cpu intensity (per cycle) with the number of cycles
    let carbon_emission: Mass = cpu_embodied_emission * track.cpu_track.cpu_cycles as f64;

    println!("{:?}", carbon_emission.get::<microgram>());
}
```

Which generates the following outputs:

```Rust
CarbonTracking {
  data: 72,
  cpu_track: {
    core: 0,
    cpu_cycles: 44
  },
  ...
}
0.000286 mg
```

## FAQ

### I need help!

Don't hesitate to file an issue or contact [@netzdoktor](https://gitlab.com/netzdoktor) via e-mail.

### How can I help?

Please have a look at the issues or open one if you feel that something is needed.

Any contributions are very welcome!

### How to cite this code?

DOI: [10.5281/zenodo.8063846](https://doi.org/10.5281/zenodo.8063846)

### Aren't there similar projects?

Yes, but we believe an operating-system service should be the future to achieve carbon awareness.

* [carbon-aware-sdk](https://github.com/Green-Software-Foundation/carbon-aware-sdk)
  * is similar in that it harmonizes information providers (centralizes credential handling),
  * in itself provides data via HTTP again and is not a system service but an SDK,
  * currently, embodied carbon of system-local resources seem out of scope (they rather focus on carbon-intensity).
* [WattTime](https://www.watttime.org/), [ElectrictyMap](https://app.electricitymaps.com/), are independent data providers for *operational carbon intensity*. While the also provide history and forecasts, they do not provide any solution for embodied emissions.

## License

Licensed under the MIT license ([LICENSE-MIT](./LICENSES/MIT.txt) or http://opensource.org/licenses/MIT).

## Acknowledgements

![CPEC](https://www.perspicuous-computing.science/wp-content/uploads/2019/11/Logo_CPEC_final_RGB.png)

This work was funded by the German Research Foundation (DFG) grant 389792660 as part of TRR&nbsp;248 &ndash; [CPEC](https://perspicuous-computing.science).

## Citation

Cite this work as

```bibtex
@inproceedings{carbond:2023:hotcarbon,
    title={carbond: An Operating-System Daemon for Carbon Awareness},
    author={Schmidt, Andreas and Stock, Gregory and Ohs, Robin and Gerhorst, Luis and Herzog, Benedict and Hönig, Timo},
    booktitle={2nd Workshop on Sustainable Computer Systems (HotCarbon)},
    address={Boston, MA, USA},
    month=7,
    doi={https://doi.org/10.1145/3604930.3605707},
    year={2023}
}
```

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be MIT licensed as above, without any additional terms or conditions.
