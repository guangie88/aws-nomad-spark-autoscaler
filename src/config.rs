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
    pub backend: Backend,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Backend {
    #[serde(rename = "aws-nomad")]
    AwsNomad(AwsNomadBackend),
}

#[derive(Deserialize, Debug)]
pub struct AwsNomadBackend {
    nomad_job_per_aws_ec2: u32,
}
