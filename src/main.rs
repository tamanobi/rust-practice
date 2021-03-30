use std::collections::HashMap;

mod indexing_tuple;

fn main() {
    let mut book = HashMap::new();

    book.insert("hogehoge".to_string(), "My favorite".to_string());
    book.insert("fugafuga".to_string(), "My favorite".to_string());
    dbg!(&book);
    indexing_tuple::perform();
    // fruit_basket();
}

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new(); // TODO: declare your hash map here.

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket here.

    basket
}



#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
