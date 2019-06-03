use crate::prelude::*;
use std::time::Instant;

pub fn log_request(guard: ContextGuard) -> AsyncResult<()> {
    let ctx = ensure_fut!(guard.metadata());
    info!("Request started to {:?}", ctx.url);
    Ok(()).fut()
}

pub fn log_result(guard: ContextGuard) -> ContextGuard {
    if let Ok(ctx) = guard.metadata() {
        let duration = Instant::now().duration_since(ctx.started);
        let total_ms = u64::from(duration.subsec_millis()) + (duration.as_secs() * 1000);
        info!("Request processed in {:?}ms", total_ms);
    }
    guard
}

pub fn log_error(err: ModuleError) -> ModuleError {
    if let Ok(ctx) = err.context.metadata() {
        let duration = Instant::now().duration_since(ctx.started);
        let total_ms = u64::from(duration.subsec_millis()) + (duration.as_secs() * 1000);
        warn!("Request failed with error: {} after {:?}ms", err.error, total_ms);
    }
    err
}
