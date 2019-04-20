mod dispatcher;
mod transformers;
mod util;

use crate::app::KatalystEngine;
use crate::config::builder::HandlerBuilder;
use crate::expression::*;
use crate::modules::*;
use crate::prelude::*;
use futures::future::*;
use futures::Future;
use http::Method;
use std::collections::HashMap;
use transformers::DownstreamTransformer;
pub use util::*;

#[derive(Debug)]
pub struct HostDispatcher {
    pub host: String,
    pub path: Expression,
    pub method: Option<Method>,
    pub query: Option<HashMap<String, Expression>>,
    pub headers: Option<HashMap<String, Expression>>,
    pub body: Option<Expression>,
}

#[derive(Debug)]
pub struct HostModule {}

impl Module for HostModule {
    fn name(&self) -> &'static str {
        "host"
    }

    fn module_type(&self) -> ModuleType {
        ModuleType::RequestHandler
    }

    fn build(
        &self,
        engine: Arc<KatalystEngine>,
        config: &ModuleConfig,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        match config {
            ModuleConfig::RequestHandler(config) => match config {
                HandlerBuilder::Host {
                    host,
                    path,
                    method,
                    query,
                    headers,
                    body,
                } => {
                    let providers = engine.locate::<Compiler>()?;
                    let method = match method {
                        Some(m) => Some(Method::from_bytes(m.to_uppercase().as_bytes())?),
                        None => None,
                    };
                    let body = match body {
                        Some(bod) => Some(bod.as_str()),
                        None => None,
                    };
                    Ok(Arc::new(HostDispatcher {
                        host: host.to_owned(),
                        path: providers.compile_template(Some(path.as_str()))?,
                        method,
                        query: providers.compile_template_map(query)?,
                        headers: providers.compile_template_map(headers)?,
                        body: providers.compile_template_option(body)?,
                    }))
                }
                _ => Err(ConfigurationFailure::InvalidResource),
            },
            _ => Err(ConfigurationFailure::InvalidResource),
        }
    }
}

impl ModuleDispatch for HostDispatcher {
    fn dispatch(&self, ctx: Context) -> ModuleResult {
        Box::new(
            result(self.prepare(ctx))
                .and_then(HostDispatcher::send)
                .map(HostDispatcher::clean_response),
        )
    }
}

impl HostDispatcher {
    pub fn transformer(
        &self,
        ctx: &Context,
        lease_str: String,
    ) -> Result<DownstreamTransformer, RequestFailure> {
        let mut uri = lease_str;
        uri.push_str(&self.path.render(ctx)?);
        if let Some(query) = &self.query {
            uri.push_str("?");
            for (key, val) in query.iter() {
                uri.push_str(&key);
                uri.push_str("=");
                uri.push_str(&val.render(&ctx)?);
                uri.push_str("&");
            }
            uri.truncate(uri.len() - 1);
        };

        let method = self.method.clone();

        let headers = match &self.headers {
            Some(h) => Some(
                h.iter()
                    .map(|(key, val)| Ok((key.to_string(), val.render(ctx)?)))
                    .collect::<Result<HashMap<String, String>, RequestFailure>>()?,
            ),
            None => None,
        };

        let body = match &self.body {
            Some(b) => Some(b.render(&ctx)?),
            None => None,
        };

        Ok(DownstreamTransformer {
            uri,
            method,
            headers,
            body,
        })
    }
}