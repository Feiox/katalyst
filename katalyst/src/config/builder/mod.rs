mod hosts;
mod listener;
mod module;
mod path;
mod routes;

pub use crate::instance::*;
pub use hosts::HostsBuilder;
pub use listener::ListenerBuilder;
pub use module::ModuleBuilder;
pub use path::PathBuilder;
pub use routes::RouteBuilder;

use crate::app::Katalyst;
use crate::error::ConfigurationFailure;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub trait Builder<T> {
    fn build(&self, engine: Arc<Katalyst>) -> Result<T, ConfigurationFailure>;
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct KatalystBuilder {
    hosts: HashMap<String, HostsBuilder>,
    routes: Vec<RouteBuilder>,
    listener: ListenerBuilder,
}

impl KatalystBuilder {
    pub fn build(self, engine: Arc<Katalyst>) -> Result<Instance, ConfigurationFailure> {
        //build routes...
        let mut all_routes = vec![];
        for route in self.routes.iter() {
            all_routes.push(Arc::new(route.build(engine.clone())?));
        }

        //final result
        Ok(Instance {
            hosts: self.hosts.build(engine.clone())?,
            routes: all_routes,
            listener: self.listener.build(engine.clone())?,
        })
    }
}
