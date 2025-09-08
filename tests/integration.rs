use sqlx::PgPool;
use rust_api_template::db;

#[tokio::test]
async fn test_insert_user() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    let pool = PgPool::connect(&database_url).await.unwrap();
    let user = db::insert_user(&pool, "test_user").await.unwrap();
    assert_eq!(user.name, "test_user");
    assert!(!user.id.is_nil());
}
