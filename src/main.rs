use std::collections::HashMap;

fn main() {
    let mut book = HashMap::new();

    book.insert("hogehoge".to_string(), "My favorite".to_string());
    book.insert("fugafuga".to_string(), "My favorite".to_string());
    dbg!(&book);
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
