use sqlx::PgPool;
use uuid::Uuid;
use crate::db;

#[tokio::test]
async fn test_insert_user() {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let user = db::insert_user(&pool, "test_user").await.unwrap();
    assert_eq!(user.name, "test_user");
    assert!(!user.id.is_nil());
}
