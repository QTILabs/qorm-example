use sqlx::SqlitePool;
use sqlx_sqlite::{init, query};

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    init(&pool).await;
    let res = query(&pool).await;
    for item in res {
        println!("{:?}", item);
    }
}
