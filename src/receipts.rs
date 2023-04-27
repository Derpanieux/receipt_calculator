use std::collections::HashMap;
use std::fmt;
use std::ops::Add;

pub struct Receipt {
    people: Vec<Person>,
    items: Vec<Item>
}
struct Person {
    name: String,
    items: Vec<Item>,
    total: Cost,
}
struct Item {
    name: String,
    cost: Cost,
    shares: HashMap<Person, f64>,
    tax_rate: f64,
    tip_rate: f64,
}
struct Cost {
    pub cents: f64,
}
impl Cost {
    fn new(c:f64) -> Cost {
        return Cost{ cents: c };
    }
    fn to_string(&self) -> String{
        let mut str = String::new();
        let ucents = self.cents as u32;
        let dollars:u32 = ucents/100;
        str += &dollars.to_string();
        str.push('.');
        let cents_only:u32 = ucents % 100;
        if cents_only < 10 {
            str.push('0');
        }
        str += &cents_only.to_string();
        return str;
    }
}

impl fmt::Display for Cost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.to_string());
    }
}
impl Item {
    fn total(&self) -> Cost{
        return Cost::new(self.cost.cents * self.tax_rate * self.tip_rate);
    }
    fn cost_per_share(&self) -> Cost {
        let total_shares:f64 = self.shares.values().sum();
        return Cost::new(self.total().cents / total_shares);
    }
    fn cost_per_person(&self, p:&Person) -> Cost {
        //let share = self.shares.get(p).unwrap_or_default();
        return Cost::new(0.0);
    }
}