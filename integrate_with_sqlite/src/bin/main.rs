use integrate_with_sqlite::{init, query};

fn main() {
    let connection = sqlite::open(":memory:").unwrap();

    init(&connection);
    let res = query(&connection);
    for item in res {
        println!("{:?}", item);
    }
}
