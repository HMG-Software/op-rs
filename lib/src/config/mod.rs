use diff::Diff;
use dirs;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use serde_with::skip_serializing_none;
use std::path::PathBuf;

type ConfigError = Box<dyn std::error::Error>;
type ConfigResult = Result<Config, ConfigError>;

/// Configuration for the OpenProject API Client
///
/// Hierarchical configuration:
/// 1. Environment variables
/// 2. Configuration file from environment variable
/// 3. Configuration file from default path
/// 4. Default values
#[derive(Clone, Debug, Default, Deserialize, Diff)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde_inline_default]
#[skip_serializing_none]
#[diff(attr(#[derive(Debug, PartialEq)]))]
pub struct Config {
    /// Path to the configuration file
    ///
    /// if not set, it defaults to $XDG_CONFIG_HOME/openproject/config.toml
    /// if it is set, it will be used to load the configuration
    /// if it is set as Environment variable, it will be used to load the configuration
    #[serde(default = "default_path", rename = "CONFIG_PATH")]
    pub path: Option<PathBuf>,

    /// API Token
    #[serde(rename = "API_TOKEN")]
    pub token: Option<String>,

    /// User ID
    #[serde_inline_default(Option::Some(String::from("me")))]
    pub user_id: Option<String>,
}

fn default_path() -> Option<PathBuf> {
    dirs::config_dir()
        .unwrap_or_default()
        .join("openproject/config.toml")
        .into()
}

// TODO: maybe impl. Builderpattern for Config?
impl Config {
    /// Create a new configuration from environment variables
    ///
    /// Do **not** load the configuration file from the environment variable!
    pub fn from_env() -> ConfigResult {
        let config: Config = de_env::from_env_prefixed("OPENPROJECT_")?;
        Ok(config)
    }

    /// Load the configuration from a file and environment variables
    pub fn from_path(path: &PathBuf) -> ConfigResult {
        let config: Config = basic_toml::from_str(&std::fs::read_to_string(path)?)?;
        Ok(config)
    }

    /// Load configuration from file and environment variables
    ///
    /// Hierarchical configuration:
    /// 1. Environment variables
    /// 2. Configuration file from environment variable
    /// 3. Configuration file from default path
    /// 4. Default values
    pub fn load() -> Config {
        let env_config: ConfigResult = Config::from_env();
        let mut config: Config = Config::default();

        match env_config {
            Ok(env) => {
                let default = &config;
                let file_config: ConfigResult = if env.path != default.path {
                    Config::from_path(env.path.as_ref().unwrap())
                } else {
                    Config::from_path(config.path.as_ref().unwrap())
                };

                match file_config {
                    Ok(file_config) => {
                        let diff_file = config.diff(&file_config);
                        config.apply(&diff_file);
                    },
                    Err(e) => println!("Error loading file configuration: {}", e),
                }

                let diff_env = config.diff(&env);
                config.apply(&diff_env);
            },
            Err(e) => println!("Error loading environment configuration: {}", e),
        }

        config
    }
}
