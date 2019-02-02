#[macro_use]
extern crate serde_derive;

use std::error::Error;

#[derive(Deserialize, Debug)]
struct Config {
    spark_master_url: String,
}

struct WorkerInfo {
    id: String,
    host: String,
    port: u16,
    webuiaddress: String,
    cores: u32,
    coresused: u32,
    coresfree: u32,
    memory: u64,
    memoryused: u64,
    memoryfree: u64,
    state: String,
    lastheartbeat: u64,
}

fn main() -> Result<(), Box<Error>> {
    let config = envy::from_env::<Config>()?;
    println!("{}", reqwest::get(&config.spark_master_url)?.text()?);

    Ok(())
}
