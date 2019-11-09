use log::info;
use std::env;
use failure::Fallible;

use oshthings::*;

fn main() -> Fallible<()> {
    // Set the default logging verbosity
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "oshthings=info,main=info,actix_web=info");
    }

    // Initialize the logger
    env_logger::init();
    info!("Starting {}:{}", PACKAGE_NAME, PACKAGE_VERSION);
    
    let server = Server::new()?;
    server.start()?;

    info!("Done. Exiting");
    Ok(())
}