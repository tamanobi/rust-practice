mod fruit_basket;
mod indexing_tuple;
mod safe_division;
mod read_file_content;

fn main() {
    // let mut book = HashMap::new();

    // book.insert("hogehoge".to_string(), "My favorite".to_string());
    // book.insert("fugafuga".to_string(), "My favorite".to_string());
    // dbg!(&book);
    // indexing_tuple::perform();
    // fruit_basket::perform();
    dbg!(&safe_division::perform(5.0, 1.0));
    dbg!(&safe_division::perform(5.0, 0.0));
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn read_file_content() {
        use super::read_file_content;
        use std::path::PathBuf;

        assert!(read_file_content::perform(PathBuf::from("src/main.rs")).is_ok());
        assert!(read_file_content::perform(PathBuf::from("non-existent-file.txt")).is_err());
    }
}
