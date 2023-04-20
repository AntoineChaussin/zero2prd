
use std::error::Error;
use zero2prd::run;

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
    run().await
}

