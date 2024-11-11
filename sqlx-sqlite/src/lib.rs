mod testing;

use qorm::{Bind, Builder};
use sqlx::{Pool, Sqlite};

#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub is_active: bool,
}

pub async fn init(pool: &Pool<Sqlite>) {
    sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, is_active BOOL NOT NULL)").execute(pool).await.unwrap();
    sqlx::query("INSERT INTO users (name, is_active) VALUES ('foo', true)")
        .execute(pool)
        .await
        .unwrap();
}

pub async fn query(pool: &Pool<Sqlite>) -> Vec<User> {
    let mut builder = Builder::new("users", None);
    builder.where_raw("name = ?");
    builder.bind_raw(Bind::String("foo".to_string()));
    builder.where_raw("is_active = ?");
    builder.bind_raw(Bind::Int(1));
    let (sql, binds) = builder.to_sql_with_bind();

    let mut query = sqlx::query_as(sql.as_str());
    for item in binds {
        query = match item {
            Bind::Null => query.bind::<Option<i32>>(None),
            Bind::String(x) => query.bind(x),
            Bind::Int(x) => query.bind(x),
            Bind::Bool(x) => query.bind(x),
            Bind::Raw(x) => query.bind(x),
        }
    }

    let sqlx_res: Vec<(i32, String, bool)> = query.fetch_all(pool).await.unwrap();
    let mut res: Vec<User> = vec![];
    for item in sqlx_res {
        res.push(User {
            id: item.0,
            name: item.1,
            is_active: item.2,
        });
    }
    res
}
