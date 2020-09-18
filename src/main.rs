use {
    lambda_http::{lambda, Request},
    lambda_runtime::{Context, error::HandlerError},
    serde_json::json,
};

fn main() {
    lambda!(handler);
}

// -> std::result::Result<Response<Body>, HandlerError> {
fn handler(_req: Request, _ctx: Context) -> std::result::Result<serde_json::Value, HandlerError> {
    Ok(json!({
        "message" : "hello rust!",
    }))
}
