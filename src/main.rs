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
    for &index in [0, 1, 99].iter() {
        match fruits.get(index) {
            Option::Some(name) => println!("{}はおいしい", name),
            Option::None => println!("ないよ"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
