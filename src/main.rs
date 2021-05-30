#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::env;

use log::Level;
use tide::{Request, Response};

async fn whoami(_req: Request<()>) -> tide::Result<Response> {
    Ok("ltoddy".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    env::set_var("RUST_LOG", Level::Info.as_str());
    env_logger::init();

    let mut app = tide::new();
    app.with(driftwood::ApacheCommonLogger);

    app.at("/whoami").get(whoami);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
