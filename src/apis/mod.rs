use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod account_api;
pub mod actions_api;
pub mod apps_api;
pub mod billing_api;
pub mod block_storage_api;
pub mod block_storage_actions_api;
pub mod cdn_endpoints_api;
pub mod certificates_api;
pub mod class1_click_applications_api;
pub mod container_registry_api;
pub mod databases_api;
pub mod domain_records_api;
pub mod domains_api;
pub mod droplet_actions_api;
pub mod droplets_api;
pub mod firewalls_api;
pub mod floating_ip_actions_api;
pub mod floating_ips_api;
pub mod functions_api;
pub mod image_actions_api;
pub mod images_api;
pub mod kubernetes_api;
pub mod load_balancers_api;
pub mod monitoring_api;
pub mod project_resources_api;
pub mod projects_api;
pub mod regions_api;
pub mod reserved_ip_actions_api;
pub mod reserved_ips_api;
pub mod ssh_keys_api;
pub mod sizes_api;
pub mod snapshots_api;
pub mod tags_api;
pub mod uptime_api;
pub mod vpcs_api;

pub mod configuration;
