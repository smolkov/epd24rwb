use serde::{Deserialize, Serialize};
use async_std::io;

/// Configuration
 #[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {


}




pub async fn read(_path: &str) -> io::Result<Config> {
    Ok(Config{})
}


pub async fn write(_config: Config) ->io::Result<()> {

    Ok(())
}
