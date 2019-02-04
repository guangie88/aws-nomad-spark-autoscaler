#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;

use failure::Error;
use reqwest::Url;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use structopt::StructOpt;
use tokio::prelude::*;
use tokio::timer::Interval;

mod config;
use config::{Args, Backend, Config};

mod structs;
use structs::{InstanceState, MasterInfo};

fn check_and_schedule(
    _instant: Instant,
    config: &Config,
    spark_master_json_url: &Url,
    index: &Mutex<u64>,
) -> Result<(), tokio::timer::Error> {
    let invoke = || -> Result<(), Error> {
        // Get the Spark Master info
        let spark_master_json_raw =
            reqwest::get((*spark_master_json_url).clone())?.text()?;

        let spark_master_info: MasterInfo =
            serde_json::from_str(&spark_master_json_raw)?;

        let worker_infos = &spark_master_info.workers;

        // Get core usage
        let (total_core_count, core_used_count): (u32, u32) = worker_infos
            .iter()
            .filter(|w| w.state == InstanceState::Alive)
            .map(|w| (w.cores, w.cores_used))
            .fold((0, 0), |(tc, cu), (ptc, pcu)| (tc + ptc, cu + pcu));

        // Check for scaling to activate
        let cores_used_ratio = core_used_count as f64 / total_core_count as f64;
        let upscale_ratio = config.cores_used_scaling_ratio;
        let downscale_ratio =
            config.cores_used_scaling_ratio * config.cores_used_scaling_ratio;

        if dbg!(cores_used_ratio >= upscale_ratio) {
            // Scale up
            // TODO
            // if let Backend::AwsNomad(_) = &config.backend {

            // }
        } else if dbg!(cores_used_ratio < downscale_ratio) {
            // Scale down
            // TODO
            // if let Backend::AwsNomad(_) = &config.backend {

            // }
        }

        // Possible TODO, poisoned mutex
        let mut index = index.lock().unwrap();

        println!("Index: {}", *index);
        println!("- Total core count: {}", total_core_count);
        println!("-  Core used count: {}", core_used_count);

        *index += 1;
        Ok(())
    };

    // TODO: Fix unwrap here
    Ok(invoke().unwrap())
}

fn main() -> Result<(), Error> {
    // Initialization section
    let args = Args::from_args();

    let config: Config = toml::from_str(&{
        let mut file = File::open(&args.config)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents
    })?;

    // Reference out those variables that does not require by be wrapped by Arc
    let period_ms = config.period_ms;

    let spark_master_url = reqwest::Url::from_str(&config.spark_master_url)?;
    let spark_master_json_url = spark_master_url.join("/json/")?;

    // Wrap everything with Arc (Send)
    // Mutable stuff needs to be further wrapped in Mutex
    let config = Arc::new(config);
    let spark_master_json_url = Arc::new(spark_master_json_url);
    let index = Arc::new(Mutex::new(0 as u64));

    // Tokio section
    let task = Interval::new(Instant::now(), Duration::from_millis(period_ms))
        .for_each(move |instant| {
            let config = Arc::clone(&config);
            let spark_master_json_url = Arc::clone(&spark_master_json_url);
            let index = Arc::clone(&index);

            check_and_schedule(
                instant,
                &*config,
                &*spark_master_json_url,
                &*index,
            )
        })
        .map_err(|e| println!("Tokio error: {}", e));

    tokio::run(task);
    Ok(())
}
