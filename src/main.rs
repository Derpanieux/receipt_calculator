
use crate::receipts::Receipt;
use std::io::*;
mod receipts;

fn main() {
    let mut r = Receipt::new_with_name("test".to_string());
    let tax = input_num("Enter a tax rate: ");
    r.set_tax(tax);
    let tip = input("Tip as value or as rate? ");
    if tip.contains("r") {
        let rate = input_num("Enter a tip rate: ");
        r.set_tip_percent(rate);
    }else if tip.contains("v") {
        let tot = input_num("Enter receipt total: ");
        let val = input_num("Enter tip amount: ");
        r.set_tip_percent(val/tot);
    }
    loop {
        let line = input("Enter a command: ");
        let mut tokens = line.split(" ");
        match tokens.next() {
            Some("help") => help(),
            Some("h") => help(),
            Some("item") => item(&mut r,tokens.collect()),

            _ => println!("Unrecognized command"),
        }
    }
}
fn input(text: &str) -> &str{
    let mut input = String::new();
    print!("{}", text);
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Did not enter a valid command");
    if let Some('\n')=input.chars().next_back() {
        input.pop();
    }
    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }
    return input.as_str();
}
fn input_num(text: &str) -> f64 {
    let line = input(text);
    let res = line.parse::<f64>();
    if res.is_err() {
        println!("Error parsing number. Try again.");
        return input_num(text);
    }
    let n = res.unwrap();
    return n;
}
fn help() {
    println!("Here is a list of available commands:");
    println!("help - Print out this help message.");
    println!("item [cost] [tax] [tip] - Add an item to your receipt. Cost should be the cost of the item, tax is y/n, tip is y/n.");

}
fn item(r: &mut Receipt, t: Vec<&str>) {
    //name, cost, tax, tip
    if t.len() != 4 {
        println!("Incorrect number of parameters supplied");
        return;
    }
    //parse name
    let name = t[0];

    //parse cost
    if t[1].parse::<u32>().is_err() {
        println!("Error parsing cost");
        return;
    }
    let cost:u32 = t[1].parse().unwrap();

    //parse tax
    if text_to_bool(t[2]).is_none() {
        println!("Error parsing tax");
        return;
    }
    let tax = text_to_bool(t[2]).unwrap();

    //parse tip
    if text_to_bool(t[3]).is_none() {
        println!("Error parsing tip");
        return;
    }
    let tip = text_to_bool(t[3]).unwrap();

    r.add_item(name.to_string(), cost, tax, tip);
    println!("Item added successfully");
}

fn text_to_bool(input: &str) -> Option<bool>{
    let c = input.chars().next();
    if c.is_none() {
        return None;
    }
    match c.unwrap() {
        'Y' | 'y' | 'T' | 't' => Some(true),
        'N' | 'n' | 'F' | 'f' => Some(false),
        _ => None
    }
}