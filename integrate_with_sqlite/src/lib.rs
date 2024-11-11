use qorm::{Bind, Builder};
use sqlite::{Connection, State, Value};

#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub is_active: bool,
}

pub fn init(connection: &Connection) {
    connection.execute("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, is_active BOOL NOT NULL)").unwrap();
    connection
        .execute("INSERT INTO users (name, is_active) VALUES ('foo', true)")
        .unwrap();
}

pub fn bind_to_sqlite_value(bind: Bind) -> Value {
    match bind {
        Bind::Null => Value::Null,
        Bind::String(v) => Value::String(v),
        Bind::Int(v) => Value::Integer(v.into()),
        Bind::Bool(v) => match v {
            true => Value::Integer(1),
            false => Value::Integer(0),
        },
        Bind::Raw(v) => Value::String(v),
    }
}

#[allow(clippy::match_like_matches_macro)]
pub fn query(connection: &Connection) -> Vec<User> {
    let mut builder = Builder::new("users", None);
    builder.where_raw("name = ?");
    builder.bind_raw(Bind::String("foo".to_string()));
    builder.where_raw("is_active = ?");
    builder.bind_raw(Bind::Int(1));
    let (query, binds) = builder.to_sql_with_bind();
    let mut statement = connection.prepare(query.as_str()).unwrap();
    for (idx, bind) in binds.iter().enumerate() {
        statement
            .bind((idx + 1, bind_to_sqlite_value(bind.clone())))
            .unwrap();
    }

    let mut res: Vec<User> = vec![];
    while let Ok(State::Row) = statement.next() {
        res.push(User {
            id: statement.read::<i64, _>("id").unwrap() as i32,
            name: statement.read::<String, _>("name").unwrap(),
            is_active: match statement.read::<i64, _>("is_active").unwrap() {
                1 => true,
                _ => false,
            },
        });
    }
    res
}
