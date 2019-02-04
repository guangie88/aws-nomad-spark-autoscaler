#[derive(Deserialize, Debug)]
pub struct MasterInfo {
    #[serde(rename = "completedapps")]
    pub completed_apps: Vec<CompletedAppsInfo>,
    pub workers: Vec<WorkerInfo>,
    #[serde(rename = "activedrivers")]
    pub active_drivers: Vec<String>,
    #[serde(rename = "completeddrivers")]
    pub completed_drivers: Vec<String>,
    pub status: InstanceState,
}

#[derive(Deserialize, Debug)]
pub struct CompletedAppsInfo {
    pub id: String,
    #[serde(rename = "starttime")]
    pub start_time: u64,
    pub name: String,
    pub cores: u32,
    pub user: String,
    #[serde(rename = "memoryperslave")]
    pub memory_per_slave: u64,
    #[serde(rename = "submitdate")]
    pub submit_date: String,
    pub state: AppState,
    pub duration: u64,
}

#[derive(Deserialize, Debug)]
pub struct WorkerInfo {
    pub id: String,
    pub host: String,
    pub port: u16,
    #[serde(rename = "webuiaddress")]
    pub web_ui_address: String,
    pub cores: u32,
    #[serde(rename = "coresused")]
    pub cores_used: u32,
    #[serde(rename = "coresfree")]
    pub cores_free: u32,
    pub memory: u64,
    #[serde(rename = "memoryused")]
    pub memory_used: u64,
    #[serde(rename = "memoryfree")]
    pub memory_free: u64,
    pub state: InstanceState,
    #[serde(rename = "lastheartbeat")]
    pub last_heart_beat: u64,
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum InstanceState {
    #[serde(rename = "ALIVE")]
    Alive,
    #[serde(rename = "DEAD")]
    Dead,
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum AppState {
    #[serde(rename = "WAITING")]
    Waiting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "FINISHED")]
    Finished,
}
