#[derive(Debug)]
pub enum Error {
    InternalError(String),

    ParseFloatError(std::num::ParseFloatError),

    #[cfg(target_os = "macos")]
    AppleSecurityFrameworkError(security_framework::base::Error),
    InquireError(inquire::InquireError),
    AlloyEcdsaError(alloy::signers::k256::ecdsa::Error),
    TomlDeError(toml::de::Error),
    TomlSerError(toml::ser::Error),
    YamlError(serde_yaml::Error),
    ReqwestError(reqwest::Error),
    SerdeJson(serde_json::Error),
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error::InternalError(e.to_string())
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Error::ParseFloatError(e)
    }
}

#[cfg(target_os = "macos")]
impl From<security_framework::base::Error> for Error {
    fn from(e: security_framework::base::Error) -> Self {
        Error::AppleSecurityFrameworkError(e)
    }
}

impl From<inquire::InquireError> for Error {
    fn from(e: inquire::InquireError) -> Self {
        Error::InquireError(e)
    }
}

impl From<alloy::signers::k256::ecdsa::Error> for Error {
    fn from(e: alloy::signers::k256::ecdsa::Error) -> Self {
        Error::AlloyEcdsaError(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::TomlDeError(e)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Error::TomlSerError(e)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Error::YamlError(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJson(e)
    }
}
