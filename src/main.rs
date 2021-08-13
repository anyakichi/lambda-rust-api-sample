use lambda_http::{handler, Body, IntoResponse, Request, Response};
use lambda_runtime::{Context, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

async fn func(req: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok(match req.body() {
        Body::Text(text) => text.to_string().into_response(),
        _ => Response::builder()
            .status(400)
            .body("unexpected request".into())
            .expect("failed to render response"),
    })
}
