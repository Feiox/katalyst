/*!
Instance provides details for the current running state of Katalyst.
*/

mod hosts;
mod listener;
mod route;

pub use hosts::Hosts;
pub use listener::Listener;
pub use route::Route;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct Instance {
    pub hosts: HashMap<String, Hosts>,
    pub routes: Vec<Arc<Route>>,
    pub listener: Listener,
}
