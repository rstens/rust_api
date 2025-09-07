use sqlx::PgPool;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await?;
    let users = vec!["Alice Dev", "Bob Dev", "Charlie Dev"];
    for name in users {
        let id = Uuid::new_v4();
        sqlx::query!(r#"INSERT INTO users (id, name) VALUES ($1, $2) ON CONFLICT DO NOTHING"#, id, name)
            .execute(&pool).await?;
        println!("✅ Inserted user: {}", name);
    }
    Ok(())
}
