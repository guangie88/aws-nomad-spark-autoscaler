#[derive(Deserialize, Debug)]
pub struct MasterInfo {
    pub completedapps: Vec<CompletedAppsInfo>,
    pub workers: Vec<WorkerInfo>,
    pub activedrivers: Vec<String>,
    pub completeddrivers: Vec<String>,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct CompletedAppsInfo {
    pub id: String,
    pub starttime: u64,
    pub name: String,
    pub cores: u32,
    pub user: String,
    pub memoryperslave: u64,
    pub submitdate: String,
    pub state: String,
    pub duration: u64,
}

#[derive(Deserialize, Debug)]
pub struct WorkerInfo {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub webuiaddress: String,
    pub cores: u32,
    pub coresused: u32,
    pub coresfree: u32,
    pub memory: u64,
    pub memoryused: u64,
    pub memoryfree: u64,
    pub state: String,
    pub lastheartbeat: u64,
}
