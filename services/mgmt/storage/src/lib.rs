#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2020-08-preview")]
mod package_2020_08_preview;
#[cfg(feature = "package-2020-08-preview")]
pub use package_2020_08_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-06")]
mod package_2019_06;
#[cfg(feature = "package-2019-06")]
pub use package_2019_06::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-04")]
mod package_2019_04;
#[cfg(feature = "package-2019-04")]
pub use package_2019_04::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-11")]
mod package_2018_11;
#[cfg(feature = "package-2018-11")]
pub use package_2018_11::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-07")]
mod package_2018_07;
#[cfg(feature = "package-2018-07")]
pub use package_2018_07::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-07-only")]
mod package_2018_07_only;
#[cfg(feature = "package-2018-07-only")]
pub use package_2018_07_only::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-03")]
mod package_2018_03;
#[cfg(feature = "package-2018-03")]
pub use package_2018_03::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-02")]
mod package_2018_02;
#[cfg(feature = "package-2018-02")]
pub use package_2018_02::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-10")]
mod package_2017_10;
#[cfg(feature = "package-2017-10")]
pub use package_2017_10::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-06")]
mod package_2017_06;
#[cfg(feature = "package-2017-06")]
pub use package_2017_06::{models, operations, API_VERSION};
#[cfg(feature = "package-2016-12")]
mod package_2016_12;
#[cfg(feature = "package-2016-12")]
pub use package_2016_12::{models, operations, API_VERSION};
#[cfg(feature = "package-2016-05")]
mod package_2016_05;
#[cfg(feature = "package-2016-05")]
pub use package_2016_05::{models, operations, API_VERSION};
#[cfg(feature = "package-2016-01")]
mod package_2016_01;
#[cfg(feature = "package-2016-01")]
pub use package_2016_01::{models, operations, API_VERSION};
#[cfg(feature = "package-2015-06")]
mod package_2015_06;
#[cfg(feature = "package-2015-06")]
pub use package_2015_06::{models, operations, API_VERSION};
#[cfg(feature = "package-2015-05-preview")]
mod package_2015_05_preview;
#[cfg(feature = "package-2015-05-preview")]
pub use package_2015_05_preview::{models, operations, API_VERSION};
#[cfg(feature = "profile-hybrid-2020-09-01")]
mod profile_hybrid_2020_09_01;
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub use profile_hybrid_2020_09_01::{models, operations, API_VERSION};
pub struct OperationConfig {
    pub api_version: String,
    pub client: reqwest::Client,
    pub base_path: String,
    pub token_credential: Option<Box<dyn azure_core::TokenCredential>>,
    pub token_credential_resource: String,
}
impl OperationConfig {
    pub fn new(token_credential: Box<dyn azure_core::TokenCredential>) -> Self {
        Self {
            token_credential: Some(token_credential),
            ..Default::default()
        }
    }
}
impl Default for OperationConfig {
    fn default() -> Self {
        Self {
            api_version: API_VERSION.to_owned(),
            client: reqwest::Client::new(),
            base_path: "https://management.azure.com".to_owned(),
            token_credential: None,
            token_credential_resource: "https://management.azure.com/".to_owned(),
        }
    }
}
