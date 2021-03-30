use std::collections::HashMap;

pub fn perform() -> HashMap<String, u32> {
    let mut basket = HashMap::new(); // TODO: declare your hash map here.

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 3);

    basket
}
