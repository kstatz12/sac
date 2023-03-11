pub mod cfg;
pub mod device;
pub mod dispatch;
pub mod screenshot;
pub mod helpers;

use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Ok(())
}

