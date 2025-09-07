use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_addr: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let cfg = config::Config::builder()
            .set_default("server_addr", "0.0.0.0:3000").unwrap()
            .add_source(config::Environment::default())
            .build()
            .unwrap();

        cfg.try_deserialize().expect("Invalid configuration")
    }
}
