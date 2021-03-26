use std::collections::HashMap;

fn main() {
    let mut book = HashMap::new();

    book.insert("hogehoge".to_string(), "My favorite".to_string());
    book.insert("fugafuga".to_string(), "My favorite".to_string());
    dbg!(&book);
    indexing_tuple()
}

fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;

    assert_eq!(
    2, second,
    "This is not the 2nd number in the tuple: {}",
    second
    )
}


#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
