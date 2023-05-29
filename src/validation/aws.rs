use validator::ValidationError;

use super::check_with;

pub fn check_region(region: &str) -> Result<(), ValidationError> {
    check_with(
        region,
        r#"^([a-z]{2}-(central|(north|south)?(east|west)?)-\d)|((ap|ca|cn|eu|sa|us)-(central|(north|south)?(east|west)?)-\d)|((me|af|ap|eu|sa)-(south|north)?(east|west)?-\d)|((us-gov)-(east|west)-\d)$"#,
        "Not a valid AWS Region",
    )
}
