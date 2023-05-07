use std::error::Error;
use zero2prd::startup::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await
}
