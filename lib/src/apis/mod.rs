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

impl<T> fmt::Display for Error<T> {
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

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                },
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

pub mod actions_capabilities_api;
pub mod activities_api;
pub mod attachments_api;
pub mod budgets_api;
pub mod categories_api;
pub mod collections_api;
pub mod configuration_api;
pub mod custom_actions_api;
pub mod custom_options_api;
pub mod documents_api;
pub mod file_links_api;
pub mod forms_api;
pub mod grids_api;
pub mod groups_api;
pub mod help_texts_api;
pub mod meetings_api;
pub mod memberships_api;
pub mod news_api;
pub mod notifications_api;
pub mod o_auth2_api;
pub mod posts_api;
pub mod previewing_api;
pub mod principals_api;
pub mod priorities_api;
pub mod projects_api;
pub mod queries_api;
pub mod query_columns_api;
pub mod query_filter_instance_schema_api;
pub mod query_filters_api;
pub mod query_operators_api;
pub mod query_sort_bys_api;
pub mod relations_api;
pub mod revisions_api;
pub mod roles_api;
pub mod root_api;
pub mod schemas_api;
pub mod statuses_api;
pub mod time_entries_api;
pub mod time_entry_activities_api;
pub mod types_api;
pub mod user_preferences_api;
pub mod users_api;
pub mod values_property_api;
pub mod versions_api;
pub mod views_api;
pub mod wiki_pages_api;
pub mod work_packages_api;
pub mod work_schedule_api;

pub mod configuration;
