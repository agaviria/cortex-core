use config::{Config, ConfigError, Environment, File};
use dotenv;

static DEFAULT_PORT: u16 = 8000;

// Object initialization for application settings
#[derive(Debug, Deserialize)]
pub struct Settings {
  pub debug: bool,
  pub port: u16,
  pub db_url: String,
  pub db_max_pool_size: Option<u32>,
}

impl Default for Settings {
  fn default() -> Settings {
    dotenv::dotenv().ok();
    let db_path = env::var("DATABASE_URL").expect(
      "DATABASE_URL must be set in .env file",
      );

    Settings {
      debug: false,
      port: DEFAULT_PORT,
      db_url: db_path,
      db_max_pool_size: 2,
    }
  }
}

impl Settings {
  // Load settings from the config file if supplied, then the environment.
  pub fn with_config_file(filename: &Option<String>) ->
    Result<Self, ConfigError> {
      let mut cfg = Config::default();

      cfg.set_default("debug", false)?;
      cfg.set_default("port", DEFAULT_PORT as i64)?;

      // Merge the config file if supplied
      if let Some(config_file) = filename {
        cfg.merge(File::with_name(config_file))?;
      }

      // Merge the env variable overrides
      cfg.merge(Environment::with_prefix("sync"))?;
      cfg.try_into()
    }
}
