use glitch::glitch;
use jemallocator;
use lambda_http::handler;
use lambda_http::Body;
use lambda_http::{IntoResponse, Request};
use lambda_runtime::{self, Context, Error};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

async fn apply_glitch(mut req: Request, _c: Context) -> Result<impl IntoResponse, Error> {
    let payload = req.body_mut();
    match payload {
        Body::Binary(image) => {
            glitch(image);
            Ok(image.to_owned())
        }
        // Ideally you want to handle Text and Empty cases too.
        // We use a special macro unimplemented!() that prevents the compiler from failing without all cases handled.
        _ => unimplemented!(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler(apply_glitch);
    lambda_runtime::run(func).await?;
    Ok(())
}
