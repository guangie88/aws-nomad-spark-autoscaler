#[macro_use]
extern crate serde_derive;

use failure::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use structopt::StructOpt;

mod structs;
use structs::MasterInfo;

#[derive(StructOpt, Debug)]
#[structopt(name = "Sparkler args (for Spark standalone autoscaler)")]
struct Args {
    #[structopt(short = "c", long = "conf", default_value = ".sparkler")]
    config: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    spark_master_url: String,
}

fn main() -> Result<(), Error> {
    let args = Args::from_args();

    let config: Config = toml::from_str(&{
        let mut file = File::open(&args.config)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents
    })?;

    let spark_master_url = reqwest::Url::from_str(&config.spark_master_url)?;
    let spark_master_json_url = spark_master_url.join("/json/")?;
    let spark_master_json_raw = reqwest::get(spark_master_json_url)?.text()?;

    let spark_master_info: MasterInfo = serde_json::from_str(&spark_master_json_raw)?;
    let worker_infos = &spark_master_info.workers;

    println!("{:#?}", worker_infos);

    let (total_core_count, core_used_count): (u32, u32) = worker_infos
        .iter()
        .map(|w| (w.cores, w.coresused))
        .fold((0, 0), |(tc, cu), (ptc, pcu)| (tc + ptc, cu + pcu));

    println!("Total core count: {}", total_core_count);
    println!(" Core used count: {}", core_used_count);

    Ok(())
}
