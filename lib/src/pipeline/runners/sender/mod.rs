use crate::pipeline::*;
use crate::state::KatalystState;
use futures::future::*;
use futures::Future;

#[derive(Default)]
pub struct Sender {}

impl Pipeline for Sender {
    fn name(&self) -> &'static str {
        "sender"
    }

    fn process(&self, mut state: PipelineState, _config: &KatalystState) -> AsyncPipelineResult {
        let dsr = match state.downstream.request {
            Some(s) => {
                state.downstream.request = None;
                s
            }
            None => {
                return Box::new(err::<PipelineState, KatalystError>(
                    KatalystError::Unavailable,
                ));
            }
        };
        let res = state.client.request(dsr);
        Box::new(res.then(|response| match response {
            Ok(r) => {
                state.upstream.response = Some(r);
                ok::<PipelineState, KatalystError>(state)
            }
            Err(e) => {
                warn!("Could not send upstream request! Caused by: {:?}", e);
                err::<PipelineState, KatalystError>(KatalystError::GatewayTimeout)
            }
        }))
    }
}
