use std::path::PathBuf;
use serde::{Deserialize, Deserializer};


pub const SCHEME: &str = env!("WOZ_WEB_SCHEME");
pub const NETLOC: &str = env!("WOZ_WEB_NETLOC");
pub const IDENTITY_POOL_ID: &str = env!("WOZ_IDENTITY_POOL_ID");
pub const IDENTITY_POOL_URL: &str = env!("WOZ_IDENTITY_POOL_URL");
pub const CLIENT_ID: &str = env!("WOZ_CLIENT_ID");
pub const S3_BUCKET_NAME: &str = env!("WOZ_S3_BUCKET_NAME");

#[derive(Debug, Serialize)]
pub enum Lib {
    WasmBindgen,
    StdWeb,
    Unknown(String)
}

impl<'de> Deserialize<'de> for Lib {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "wasm-bindgen" => Lib::WasmBindgen,
            "std-web" => Lib::StdWeb,
            _ => Lib::Unknown(s),
        })
    }
}

#[derive(Debug, Serialize)]
pub enum Environment {
    Release,
    Development,
    Unknown(String)
}

impl<'de> Deserialize<'de> for Environment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "release" => Environment::Release,
            "development" => Environment::Development,
            _ => Environment::Unknown(s),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ProjectId(pub String);

impl<'de> Deserialize<'de> for ProjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        if s.chars().all(char::is_alphanumeric) {
            Ok(ProjectId(s))
        } else {
            Err(serde::de::Error::custom(String::from("must be alphanumeric")))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub project_id: ProjectId,
    pub lib: Option<Lib>,
    pub name: String,
    pub short_name: Option<String>,
    pub env: Option<Environment>,
    pub wasm_path: PathBuf
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project_id: ProjectId(String::from("default")),
            lib: Some(Lib::WasmBindgen),
            name: String::from("My App"),
            short_name: Some(String::from("App")),
            env: Some(Environment::Release),
            wasm_path: PathBuf::new(),
        }
    }
}