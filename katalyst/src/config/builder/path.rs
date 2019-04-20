use super::Builder;
use crate::app::KatalystEngine;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PathBuilder {
    Regex {
        pattern: String,
    },
    Template {
        template: String,
    },
    Exact {
        path: String,
        #[serde(default)]
        sensitive: bool,
    },
}

impl Default for PathBuilder {
    fn default() -> Self {
        PathBuilder::Exact {
            path: "/".to_string(),
            sensitive: false,
        }
    }
}

impl Builder<String> for PathBuilder {
    fn build(&self, e: Arc<KatalystEngine>) -> Result<String, ConfigurationFailure> {
        match self {
            PathBuilder::Regex { pattern } => Ok(pattern.to_string()),
            PathBuilder::Template { template } => Ok({
                let compiler = e.locate::<Compiler>()?;
                let mut result = String::new();
                result.push_str("^");
                let cmp = compiler.compile_template(Some(template))?;
                let ctx = Context::default();
                let rnd = cmp.render(&ctx).map_err(|_| {
                    ConfigurationFailure::InvalidExpressionArgs(
                        "Path template could not be rendered",
                    )
                })?;
                result.push_str(&rnd);
                result
            }),
            PathBuilder::Exact { path, sensitive } => Ok({
                let mut result = String::new();
                result.push_str("^");
                if !*sensitive {
                    result.push_str("(?i:");
                }
                result.push_str("(?P<root_uri>");
                let escaped = regex::escape(path);
                result.push_str(&escaped);
                if !*sensitive {
                    result.push_str(")")
                }
                result.push_str(")$");
                result
            }),
        }
    }
}