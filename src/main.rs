#[macro_use]
extern crate serde_derive;

use std::error::Error;

#[derive(Deserialize, Debug)]
struct Config {
    spark_master_url: String,
}

#[derive(Deserialize, Debug)]
struct MasterInfo {
    completedapps: Vec<CompletedAppsInfo>,
    workers: Vec<WorkerInfo>,
    activedrivers: Vec<String>,
    completeddrivers: Vec<String>,
    status: String,
}

#[derive(Deserialize, Debug)]
struct CompletedAppsInfo {
    id: String,
    starttime: u64,
    name: String,
    cores: u32,
    user: String,
    memoryperslave: u64,
    submitdate: String,
    state: String,
    duration: u64,
}

#[derive(Deserialize, Debug)]
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
    let spark_master_json_raw = reqwest::get(&config.spark_master_url)?.text()?;

    // println!("{}", spark_master_json_raw);

    let spark_master_info: MasterInfo = serde_json::from_str(&spark_master_json_raw)?;
    let worker_infos = &spark_master_info.workers;

    println!("{:#?}", worker_infos);

    let (total_core_count, core_used_count): (u32, u32) = worker_infos.iter()
        .map(|w| (w.cores, w.coresused))
        .fold((0, 0), |(tc, cu), (ptc, pcu)| (tc + ptc, cu + pcu));

    println!("Total core count: {}", total_core_count);
    println!(" Core used count: {}", core_used_count);

    Ok(())
}
