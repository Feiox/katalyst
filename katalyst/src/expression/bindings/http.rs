use crate::prelude::*;

#[derive(ExpressionBinding)]
#[expression(name = "http", bind = method)]
#[expression(bind = ip)]
#[expression(bind = path)]
#[expression(bind = query)]
#[expression(bind = query_param)]
#[expression(bind = header)]
#[expression(bind = matched)]
pub struct Http;

impl Http {
    fn method(guard: &RequestContext, _: &[ExpressionArg]) -> ExpressionResult {
        Ok(guard.method().as_str().into())
    }

    fn ip(guard: &RequestContext, _: &[ExpressionArg]) -> ExpressionResult {
        Ok(guard.metadata()?.remote_ip.to_owned().into())
    }

    fn path(guard: &RequestContext, _: &[ExpressionArg]) -> ExpressionResult {
        Ok(guard.metadata()?.url.path().into())
    }

    fn query(guard: &RequestContext, _: &[ExpressionArg]) -> ExpressionResult {
        Ok(guard.metadata()?.url.query().unwrap_or_default().into())
    }

    fn query_param(guard: &RequestContext, args: &[ExpressionArg]) -> ExpressionResult {
        let metadata = guard.metadata()?;
        let name = args[0].render(guard)?;
        let res = metadata.url.query_pairs().find(|q| q.0 == name);
        res.map_or_else(
            || fail!(BAD_REQUEST, format!("Expected query parameter {}", name)),
            |v| Ok(v.1.to_string().into()),
        )
    }

    fn header(guard: &RequestContext, args: &[ExpressionArg]) -> ExpressionResult {
        let arg = &args[0].render(guard)?;
        let hdr = guard
            .header(arg)
            .ok_or_else(|| fail!(_ BAD_REQUEST, format!("Expected header parameter {}", arg)))?;
        Ok(hdr.into())
    }

    fn matched(guard: &RequestContext, args: &[ExpressionArg]) -> ExpressionResult {
        let value = args[0].render(guard)?;
        let result = guard.get_match()?.get_value(&value)?;
        Ok(result.into())
    }
}
