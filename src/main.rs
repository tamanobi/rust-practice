use std::collections::HashMap;

mod fruit_basket;
mod indexing_tuple;

fn main() {
    // let mut book = HashMap::new();

    // book.insert("hogehoge".to_string(), "My favorite".to_string());
    // book.insert("fugafuga".to_string(), "My favorite".to_string());
    // dbg!(&book);
    // indexing_tuple::perform();
    // fruit_basket::perform();
    let fruits = vec!["banana", "apple", "dragon_fruit"];
    dbg!(&fruits.get(0));
    let non_existent = fruits.get(1000);
    dbg!(&non_existent);
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
