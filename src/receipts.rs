use std::collections::HashMap;
pub struct Receipt {
    people: Vec<Person>,
    items: Vec<Item>
}
struct Person {
    name: String,
    items: Vec<Item>
}
struct Item {
    name: String,
    cost: u32,
    total_shares: f64,
    shares: HashMap<Person, f64>
}