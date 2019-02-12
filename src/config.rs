use aws_nomad::AwsNomadBackend;

#[derive(StructOpt, Debug)]
#[structopt(name = "Sparkler args (for Spark standalone autoscaler)")]
pub struct Args {
    #[structopt(short = "c", long = "conf", default_value = ".sparkler")]
    pub config: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub spark_master_url: String,
    pub period_ms: u64,
    pub cores_used_scaling_ratio: f64,
    pub backend: Backend,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Backend {
    #[serde(rename = "aws-nomad")]
    AwsNomad(AwsNomadBackend),
}
