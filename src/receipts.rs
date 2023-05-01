use std::collections::HashMap;
use std::fmt;
use std::ops::*;
use std::hash::{Hash, Hasher};

pub struct Receipt {
    people: Vec<Person>,
    items: Vec<Item>
}

#[derive(Clone)]
pub struct Person {
    name: String,
    items: Vec<Item>,
}
impl Person{
    pub fn new() -> Self {
        return Self::new_with_name("".to_string());
    }
    pub fn new_with_name(name: String) -> Self {
        return Self {
            name,
            items: Vec::new()
        }
    }
    fn total(&self) -> Cost {
        let mut tot = Cost(0.0);
        for item in self.items.iter() {
            tot += item.cost_per_person(self);
        }
        return tot;
    }
    pub fn to_string(&self) -> String{
        let mut str = String::new();
        str += &self.name;
        str += "\n";
        for i in self.items.iter() {
            str += &i.name;
            str += ", ";
        }
        return str;
    }
}
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}
impl Eq for Person {}
impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Clone)]
pub struct Item {
    name: String,
    cost: Cost,
    shares: HashMap<Person, f64>,
    tax_rate: f64,
    tip_rate: f64,
}
impl Item {
    pub fn new() -> Self {
        return Self::new_with_all(String::new(), 0.0,1.0, 1.0);
    }
    pub fn new_with_name_cost(name: String, cost: f64) -> Self {
        return Self::new_with_all(name, cost, 1.0, 1.0);
    }
    pub fn new_with_all(name: String, cost: f64, tax_rate: f64, tip_rate: f64) -> Self {
        return Self {
            name,
            cost: Cost(cost),
            shares: HashMap::new(),
            tax_rate,
            tip_rate
        }
    }
    pub fn set_share(&mut self, p: &Person, s: &f64) {
        self.shares.insert(p.clone(), s.clone());
    }
    pub fn total(&self) -> Cost{
        let multiplier = self.tax_rate * self.tip_rate;
        return self.cost.clone() * Cost(multiplier);
    }
    fn cost_per_share(&self) -> Cost {
        let total_shares:f64 = self.shares.values().sum();
        return self.total() / Cost(total_shares);
    }
    fn cost_per_person(&self, p:&Person) -> Cost {
        let share = Cost(*self.shares.get(p).unwrap());
        return self.cost_per_share() * share;
    }
    fn all_costs_per_person(&self) -> HashMap<Person, Cost> {
        let mut map = HashMap::new();
        let per_share = self.cost_per_share();
        for p in self.shares.iter() {
            map.insert((*p.0).clone(), per_share.clone() * Cost(*p.1));
        }
        return map;
    }
    fn to_string(&self) -> String{
        let mut str = String::new();
        str += "Item: ";
        str += &self.name;
        str += " ";
        str += &self.cost.to_string();
        str += "\n";
        for i in self.shares.iter() {
            str += &i.0.name;
            str += "(";
            str += &i.1.to_string();
            str += "), ";
        }
        return str;
    }
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.to_string());
    }
}

struct Cost (f64);
impl Cost {
    fn to_string(&self) -> String{
        let mut str = String::new();
        str.push('$');
        let ucents = self.0 as u32;
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
impl Add for Cost {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return Self(self.0 + other.0);
    }
}
impl AddAssign for Cost {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl Sub for Cost {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        return Self(self.0 - other.0);
    }
}
impl Mul for Cost {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        return Self(self.0 * other.0);
    }
}
impl Div for Cost {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        return Self(self.0 / other.0);
    }
}
impl Clone for Cost {
    fn clone(&self) -> Self {
        return Self(self.0);
    }
}
impl PartialEq for Cost {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}
impl Eq for Cost {}
