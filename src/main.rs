#[macro_use]
extern crate serde_derive;

use failure::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use structopt::StructOpt;
use tokio::prelude::*;
use tokio::timer::Interval;

mod structs;
use structs::{Backend, InstanceState, MasterInfo};

#[derive(StructOpt, Debug)]
#[structopt(name = "Sparkler args (for Spark standalone autoscaler)")]
struct Args {
    #[structopt(short = "c", long = "conf", default_value = ".sparkler")]
    config: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    spark_master_url: String,
    period_ms: u64,
    backend: Backend,
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

    let spark_master_url = reqwest::Url::from_str(&config.spark_master_url)?;
    let spark_master_json_url = spark_master_url.join("/json/")?;

    let spark_master_json_url = Arc::new(spark_master_json_url);
    let index = Arc::new(Mutex::new(0 as u64));

    // Tokio section
    let task = Interval::new(Instant::now(), Duration::from_millis(config.period_ms))
        .for_each(move |_instant| {
            let spark_master_json_url = Arc::clone(&spark_master_json_url);
            let index = Arc::clone(&index);

            let invoke = || -> Result<(), Error> {
                let spark_master_json_raw =
                    reqwest::get((*spark_master_json_url).clone())?.text()?;
                let spark_master_info: MasterInfo = serde_json::from_str(&spark_master_json_raw)?;
                let worker_infos = &spark_master_info.workers;

                // println!("{:#?}", worker_infos);

                let (total_core_count, core_used_count): (u32, u32) = worker_infos
                    .iter()
                    .filter(|w| w.state == InstanceState::Alive)
                    .map(|w| (w.cores, w.cores_used))
                    .fold((0, 0), |(tc, cu), (ptc, pcu)| (tc + ptc, cu + pcu));

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
        })
        .map_err(|e| println!("Tokio error: {}", e));

    tokio::run(task);
    Ok(())
}
