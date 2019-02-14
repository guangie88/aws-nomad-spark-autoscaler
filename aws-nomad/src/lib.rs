#[macro_use]
extern crate serde_derive;

use failure::Error;
use reqwest;

pub mod config;
pub use config::AwsNomadBackend;

pub fn scale_up(config: &AwsNomadBackend) -> Result<(), Error> {
    let nomad_url = reqwest::Url::parse(&config.nomad_url)?;
    let spark_worker_url =
        nomad_url.join(&format!("/v1/job/{}", &config.nomad_job_name))?;

    let mut spark_worker_response = reqwest::get(spark_worker_url)?;
    let spark_worker_response_text = spark_worker_response.text()?;

    let spark_worker_info_json =
        serde_json::from_str(&spark_worker_response_text)?;

    // NEW_JOB=$(curl -s https://nomad.locus.rocks/v1/job/spark-worker | jq ".TaskGroups[].Count *= 2" | jq ".VaultToken=\"`cat ~/.vault-token`\"")
    let _str_res = json_query::run(
        r#".TaskGroups[].Count *= 2 | .VaultToken="`cat ~/.vault-token`""#,
        spark_worker_info_json,
    );

    Ok(())
}

pub fn scale_down(_config: &AwsNomadBackend) -> Result<(), Error> {
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn scale_up_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn scale_down_test() {
        assert_eq!(2 + 2, 4);
    }
}
