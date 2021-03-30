use std::collections::HashMap;

mod indexing_tuple;
mod fruit_basket;

fn main() {
    let mut book = HashMap::new();

    book.insert("hogehoge".to_string(), "My favorite".to_string());
    book.insert("fugafuga".to_string(), "My favorite".to_string());
    // dbg!(&book);
    indexing_tuple::perform();
    let basket = fruit_basket::perform();
    assert!(
        basket.len() >= 2,
        "basket must have at least three types of fruits"
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
