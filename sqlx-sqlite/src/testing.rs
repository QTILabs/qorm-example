#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use crate::{init, query};

    #[tokio::test]
    async fn test_query() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        init(&pool).await;
        let res = query(&pool).await;

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, 1);
        assert_eq!(res[0].name, "foo".to_string());
        assert!(res[0].is_active);
    }
}
