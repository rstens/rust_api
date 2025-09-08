#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_addr: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let _ = dotenvy::dotenv();
        let server_addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set (e.g., postgres://postgres:postgres@localhost:5432/rust_api)");
        Self { database_url, server_addr }
    }
}
