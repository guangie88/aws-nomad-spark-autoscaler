#[derive(Deserialize, Debug)]
pub struct AwsNomadBackend {
    nomad_job_per_aws_ec2: u32,
}
