use super::Builder;
use crate::{app::Katalyst, prelude::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// A PathBuilder for building path strings from configuration
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PathBuilder {
    /// A regex path
    Regex {
        /// The regex match string for this path
        pattern: String,
    },
    /// An expression templated path
    Template {
        /// The expression based template for this path
        template: String,
    },
    /// Exact match only path
    Exact {
        /// The exact path that should match
        path: String,
        /// Boolean indicating if the path should be case sensitive
        #[serde(default)]
        sensitive: bool,
    },
}

impl Default for PathBuilder {
    fn default() -> Self {
        PathBuilder::Exact { path: "/".to_string(), sensitive: false }
    }
}

impl Builder<String> for PathBuilder {
    fn build(&self, e: Arc<Katalyst>) -> Result<String> {
        match self {
            PathBuilder::Regex { pattern } => Ok(pattern.to_string()),
            PathBuilder::Template { template } => Ok({
                let compiler = e.get_compiler();
                let mut result = String::new();
                result.push_str("^");
                let cmp = compiler.compile_template(Some(template))?;
                let ctx = RequestContext::default();
                let rnd = cmp.render(&ctx).map_err(|e| {
                    err!(
                        ConfigurationFailure,
                        format!("Unable to parse path template {}", template),
                        e
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
