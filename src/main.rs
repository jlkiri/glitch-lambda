use glitch::glitch_replace;
use glitch::glitch_sort;
use lambda_http::handler;
use lambda_http::Body;
use lambda_http::{
    http::{HeaderValue, StatusCode},
    IntoResponse, Request, RequestExt, Response,
};
use lambda_runtime::{self, Context, Error};

use jemallocator;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
// use std::fs;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

async fn apply_glitch(mut req: Request, _c: Context) -> Result<impl IntoResponse, Error> {
    let payload = req.body_mut();
    match payload {
        Body::Binary(image) => {
            glitch(image);
            Ok(image.to_owned())
        }
        _ => unimplemented!(),
    }
}

fn glitch(image: &mut [u8]) {
    glitch_replace(image);
    glitch_sort(image);
    glitch_replace(image);
    glitch_sort(image);
    glitch_replace(image);
    glitch_sort(image);
    glitch_sort(image);
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    /* let path = std::env::args().nth(1).unwrap_or_else(|| panic!("whoops"));
    let mut image = fs::read(path)?;

    fs::write("glitched.jpg", image)?; */
    let func = handler(apply_glitch);
    lambda_runtime::run(func).await?;
    Ok(())
}
