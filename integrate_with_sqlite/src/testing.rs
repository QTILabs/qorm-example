#[cfg(test)]
mod tests {
    use crate::{init, query};

    #[test]
    fn test_query() {
        let connection = sqlite::open(":memory:").unwrap();
        init(&connection);
        let res = query(&connection);

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, 1);
        assert_eq!(res[0].name, "foo".to_string());
        assert!(res[0].is_active);
    }
}
