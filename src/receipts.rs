use std::collections::HashMap;
use std::fmt;
use std::ops::*;
use std::hash::{Hash, Hasher};

pub struct Receipt<'a> {
    name: String,
    people: Vec<Person<'a>>,
    items: Vec<Item<'a>>,
    tax_rate: f64,
    tip_rate:f64
}
impl Receipt<'_> {
    pub fn new() -> Self {
        return Self {
            name: "".to_string(),
            people: Vec::new(),
            items: Vec::new(),
            tax_rate: 1.0,
            tip_rate: 1.0,
        }
    }
    pub fn new_with_name(name: String) -> Self {
        return Self {
            name,
            people: Vec::new(),
            items: Vec::new(),
            tax_rate: 1.0,
            tip_rate: 1.0,
        }
    }
    pub fn add_person(&mut self, name: String) -> bool{
        let p = Person::new_with_name(name);
        if !self.people.contains(&p) {
            self.people.push(p);
            return true;
        }
        return false;
    }
    pub fn add_item(&mut self, name: String, cost: u32, tax: bool, tip: bool) -> bool {
        let i = Item::new_with_all(
            name,
            cost as f64,
            if tax {self.tax_rate} else {1.0},
            if tip {self.tip_rate} else {1.0});
        if !self.items.contains(&i) {
            self.items.push(i);
            return true;
        }
        return false;
    }
    pub fn set_tax(&mut self, r:f64) {
        self.tax_rate = r;
        for i in self.items.iter_mut() {
            if i.tax {
                i.tax_rate = r;
            }
        }
    }
    pub fn set_tip_percent(&mut self, r: f64) {
        self.tip_rate = r;
        for i in self.items.iter_mut() {
            if i.tip {
                i.tip_rate = r;
            }
        }
    }
    pub fn set_share(&mut self, person: usize, item: usize, share:f64) {
        self.items[item].set_share(&self.people[person], &share);
        self.people[person].add_item(&self.items[item]);
    }
    pub fn get_totals(&self) -> HashMap<String, u32> {
        let mut map = HashMap::new();
        for p in self.people.iter() {
            map.insert(p.name.clone(), p.total().0 as u32);
        }
        return map;
    }
    fn total_paid(&self) -> Cost {
        let mut sum = Cost(0.0);
        for p in self.people.iter() {
            sum += p.total();
        }
        return sum;
    }
    fn total_cost(&self) -> Cost {
        let mut sum = Cost(0.0);
        for i in self.items.iter() {
            sum += i.total();
        }
        return sum;
    }
    fn sub_total(&self) -> Cost {
        let mut sum = Cost(0.0);
        for i in self.items.iter() {
            sum += i.cost.clone();
        }
        return sum;
    }
    fn total_tip(&self) -> Cost {
        let mut t = Cost(0.0);
        for i in self.items.iter() {
            t += i.total_tip();
        }
        return t;
    }
    fn total_tax(&self) -> Cost {
        let mut t = Cost(0.0);
        for i in self.items.iter() {
            t += i.total_tax();
        }
        return t;
    }
    pub fn to_string(&self) -> String {
        let mut str = String::new();
        str += &self.name;
        str += "\n";
        for i in 0..self.items.len() {
            str += &i.to_string();
            str += ". ";
            str += &self.items[i].to_string();
        }
        str += "\n";
        for i in 0..self.people.len() {
            str += &i.to_string();
            str += ". ";
            str += &self.people[i].to_string();
        }
        str += "\nSubtotal: ";
        str += &self.sub_total().to_string();
        str += "\nTax: ";
        str += &self.total_tax().to_string();
        str += "\nTip: ";
        str += &self.total_tip().to_string();
        str += "\nTotal: ";
        str += &self.total_cost().to_string();
        str += "\nPaid: ";
        str += &self.total_paid().to_string();
        return str;
    }
}

impl fmt::Display for Receipt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.to_string());
    }
}

#[derive(Clone)]
struct Person<'a> {
    name: String,
    items: Vec<&'a Item<'a>>,
}
impl Person<'_>{
    fn new() -> Self {
        return Self::new_with_name("".to_string());
    }
    fn new_with_name(name: String) -> Self {
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
    fn to_string(&self) -> String{
        let mut str = String::new();
        str += &self.name;
        str += "(";
        str += &self.total().to_string();
        str += ")\n";
        for i in self.items.iter() {
            str += &i.name;
            str += "(";
            str += &i.cost_per_person(self).to_string();
            str += "), ";
        }
        return str;
    }
    fn add_item(&mut self, i: &Item) {
        if !self.items.contains(&i) {
            self.items.push(&i);
        }
    }
}
impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.to_string());
    }
}
impl PartialEq for Person<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}
impl Eq for Person<'_> {}
impl Hash for Person<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Clone)]
struct Item<'a> {
    name: String,
    cost: Cost,
    shares: HashMap<&'a Person<'a>, f64>,
    tax_rate: f64,
    tip_rate: f64,
    tax: bool,
    tip: bool
}
impl Item<'_> {
    fn new() -> Self {
        return Self::new_with_all(String::new(), 0.0,1.0, 1.0);
    }
    fn new_with_name_cost(name: String, cost: f64) -> Self {
        return Self::new_with_all(name, cost, 1.0, 1.0);
    }
    fn new_with_all(name: String, cost: f64, tax_rate: f64, tip_rate: f64) -> Self {
        let tax = tax_rate != 1.0;
        let tip = tip_rate != 1.0;
        return Self {
            name,
            cost: Cost(cost),
            shares: HashMap::new(),
            tax_rate,
            tip_rate,
            tax,
            tip,
        }
    }
    fn set_share(&mut self, p: &Person, s: &f64) {
        self.shares.insert(p, s.clone());
    }
    fn total(&self) -> Cost{
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
    fn all_costs_per_person(&self) -> HashMap<&Person, Cost> {
        let mut map = HashMap::new();
        let per_share = self.cost_per_share();
        for p in self.shares.iter() {
            map.insert(*p.0, per_share.clone() * Cost(*p.1));
        }
        return map;
    }
    fn total_tax(&self) -> Cost {
        return self.cost.clone() * Cost(self.tax_rate - 1.0);
    }
    fn total_tip(&self) -> Cost {
        return self.total() - self.cost.clone() - self.total_tax();
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
            str += ",";
            str += &self.cost_per_person(i.0).to_string();
            str += "), ";
        }
        return str;
    }
}
impl fmt::Display for Item<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.to_string());
    }
}
impl PartialEq for Item<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name
        && self.tip_rate == other.tip_rate
        && self.tax_rate == other.tax_rate
        && self.cost == other.cost;
    }
}
impl Eq for Item<'_> {}

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
