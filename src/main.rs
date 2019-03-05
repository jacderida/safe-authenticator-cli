use env_logger;
use log::{error, info};

use safe_auth::run;
use std::process;

fn main() {
    env_logger::init();
    info!("Starting Authenticator...");

	if let Err(e) = run() {
        error!("Auth lib error: {}", e);
        process::exit(1);
    }
}