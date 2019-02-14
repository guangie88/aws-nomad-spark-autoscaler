#[derive(Deserialize, Debug)]
pub struct AwsNomadBackend {
    pub nomad_url: String,
    pub nomad_job_name: String,
    pub nomad_job_per_aws_ec2: u32,
    pub scaling_mode: ScalingMode,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum ScalingMode {
    #[serde(rename = "doubling")]
    Doubling,
    #[serde(rename = "absolute")]
    Absolute(u32),
}
