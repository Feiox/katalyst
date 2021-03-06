use crate::prelude::*;
use unstructured::Document;

pub enum Format {
    Default,
    Json,
    Yaml,
}

impl Format {
    pub fn ext(ext: Option<&str>) -> Format {
        match ext {
            Some("yml") | Some("yaml") => Format::Yaml,
            Some("json") | Some("js") => Format::Json,
            _ => Format::Default,
        }
    }

    pub fn content_type(content_type: Option<&str>) -> Format {
        if let Some(ct) = content_type {
            match ct {
                "application/json" | "application/javascript" => Format::Json,
                "application/x-yaml" | "text/vnd.yaml" | "text/yaml" | "text/x-yaml" => {
                    Format::Yaml
                }
                _ => Format::Default,
            }
        } else {
            Format::Default
        }
    }

    pub fn parse(&self, data: &[u8]) -> Result<Document> {
        match self {
            Format::Json => Ok(serde_json::from_slice(data)?),
            Format::Yaml => Ok(serde_yaml::from_slice(data)?),
            _ => Ok(serde_json::from_slice(data).unwrap_or_else(|_| Document::default())),
        }
    }
}

pub struct Parser;

impl Parser {
    pub fn from_str<T: serde::de::DeserializeOwned>(ser: &str, f: Format) -> Result<T> {
        match f {
            Format::Json | Format::Default => serde_json::from_str(ser).map_err(|e| {
                err!(ConfigurationFailure, "Failed to parse JSON configuration file", e)
            }),
            Format::Yaml => serde_yaml::from_str(ser).map_err(|e| {
                err!(ConfigurationFailure, "Failed to parse YAML configuration file", e)
            }),
        }
    }
}
