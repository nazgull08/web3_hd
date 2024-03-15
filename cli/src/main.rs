pub mod settings;
pub mod error;

use error::Error;
use settings::Settings;

use dotenv::dotenv;


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let settings = Settings::new()?;
    println!("settings {:?}",settings);
    Ok(())
}
