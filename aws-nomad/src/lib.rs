#[macro_use]
extern crate serde_derive;

use failure::Error;

pub mod config;
pub use config::AwsNomadBackend;

pub fn scale_up(_config: &AwsNomadBackend) -> Result<(), Error> {
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
