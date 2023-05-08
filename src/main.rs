use std::error::Error;
use zero2prd::startup::{self, run};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    startup::setup_log("zero2prod".into(), "info".into(), std::io::stdout);
    run().await
}
