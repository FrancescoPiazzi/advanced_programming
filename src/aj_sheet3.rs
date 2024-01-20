#![allow(dead_code)]

fn is_it_luhn(s: String) -> bool {
    let mut sum = 0;
    let mut i = 0;
    for c in s.chars().rev() {
        if let Some(n) = c.to_digit(10) {
            sum += if i % 2 == 0 {
                n
            } else {
                let doubled = n * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            };
            i += 1;
        }
    }
    sum % 10 == 0
}

enum Fuel{
    Disel,
    Gasoline,
    LPG,
    Methane,
    Electric
}

enum IpAddr{
    Ipv4([i32; 4]),
    Ipv6([i32; 6])
}

struct Point3d{
    x: f32,
    y: f32,
    z: f32
}

use std::{collections::HashMap, hash::Hash};

#[derive(Debug, PartialEq, Eq)]
struct CarOwner{
    name: String,
    address: String,
    phone: String,
}

fn recognise_owner<'a>(s: &str, hm: &'a HashMap<&str, CarOwner>) -> Option<&'a CarOwner> {
    hm.get(s)
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Item{
    Coffee,
    Tea,
    Ham,
    Jupiter,
}

enum Coin{
    Cent,
    TwoCents,
    EighteenCents,
    Rapper,
    Euro,
    TwoEuro,
}

fn to_cents(c: Coin) -> u32{
    match c{
        Coin::Cent => {1}
        Coin::TwoCents => {2}
        Coin::EighteenCents => {18}
        Coin::Rapper => {50}
        Coin::Euro => {100}
        Coin::TwoEuro => {200}
    }
}

struct VendingMachine{
    coins: u32,
    items: HashMap<Item, usize>
}

impl VendingMachine{
    fn new(v: Vec<Item>) -> VendingMachine{
        VendingMachine{
            coins: 0, 
            // this is wrong, if the same item is present multiple times in the vector it should be added multiple times
            items: v.iter().map(|k| (*k, 1)).collect(),
        }
    }

    fn add_item(&mut self, i: Item, u: usize){
        match self.items.get_mut(&i){
            Some(x) => { *x+=u; }
            None => { self.items.insert(i, 0); }
        }
    }

    fn insert_coin(&mut self, c: Coin) -> Result<&'static str, &'static str>{
        self.coins += to_cents(c);
        return Ok("done :)");
    }

    fn get_item_price(&self, i: Item) -> u32{
        match i {
            Item::Coffee => { 47 }
            Item::Tea => { 121 }
            Item::Ham => { 37 }
            Item::Jupiter => { 183 }
        }
    }

    fn buy(&self, i:Item) -> Result<u32, &'static str>{
        match self.items.get(&i){
            Some(x) => {
                if *x == 0{
                    Err("no items left")
                } else {
                    let price = self.get_item_price(i);
                    if self.coins > price{
                        Ok(self.coins - price)
                    } else {
                        Err("not enough money")
                    }
                }
            },
            None =>  Err("item not found"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_it_luhn() {
        assert_eq!(is_it_luhn("79927398713".to_string()), true);
        assert_eq!(is_it_luhn("79927398714".to_string()), false);
        assert_eq!(is_it_luhn("79927398715".to_string()), false);
        assert_eq!(is_it_luhn("79927398716".to_string()), false);
        assert_eq!(is_it_luhn("79927398717".to_string()), false);
        assert_eq!(is_it_luhn("79927398718".to_string()), false);
        assert_eq!(is_it_luhn("79927398719".to_string()), false);
        assert_eq!(is_it_luhn("79927398710".to_string()), false);
        assert_eq!(is_it_luhn("79927398711".to_string()), false);
        assert_eq!(is_it_luhn("79927398712".to_string()), false);
    }

    #[test]
    fn test_recognise_owner() {
        let mut hm: HashMap<&str, CarOwner> = HashMap::new();
        hm.insert("AA123BB", CarOwner{
            name: "Mario Rossi".to_string(),
            address: "Via Roma 1".to_string(),
            phone: "1234567890".to_string(),
        });
        hm.insert("BB123CC", CarOwner{
            name: "Giuseppe Verdi".to_string(),
            address: "Via Verdi 1".to_string(),
            phone: "0987654321".to_string(),
        });
        hm.insert("CC123DD", CarOwner{
            name: "Giovanni Bianchi".to_string(),
            address: "Via Bianchi 1".to_string(),
            phone: "1111111111".to_string(),
        });
        assert_eq!(recognise_owner("AA123BB", &hm), Some(&CarOwner{
            name: "Mario Rossi".to_string(),
            address: "Via Roma 1".to_string(),
            phone: "1234567890".to_string(),
        }));
    }

    #[test]
    fn test_vending_machine_new() {
        let vm = VendingMachine::new(vec![Item::Coffee, Item::Tea, Item::Ham, Item::Jupiter]);
        assert_eq!(vm.coins, 0);
        assert_eq!(vm.items.get(&Item::Coffee), Some(&1));
        assert_eq!(vm.items.get(&Item::Tea), Some(&1));
        assert_eq!(vm.items.get(&Item::Ham), Some(&1));
        assert_eq!(vm.items.get(&Item::Jupiter), Some(&1));
    }

    #[test]
    fn test_vending_machine_add_item() {
        let mut vm = VendingMachine::new(vec![Item::Coffee, Item::Tea, Item::Ham, Item::Jupiter]);
        vm.add_item(Item::Coffee, 5);
        assert_eq!(vm.items.get(&Item::Coffee), Some(&6));
    }

    #[test]
    fn test_vending_machine_insert_coin() {
        let mut vm = VendingMachine::new(vec![Item::Coffee, Item::Tea, Item::Ham, Item::Jupiter]);
        assert_eq!(vm.insert_coin(Coin::Cent), Ok("done :)"));
        assert_eq!(vm.coins, 1);
    }

    #[test]
    fn test_vending_machine_get_item_price() {
        let vm = VendingMachine::new(vec![Item::Coffee, Item::Tea, Item::Ham, Item::Jupiter]);
        assert_eq!(vm.get_item_price(Item::Coffee), 47);
        assert_eq!(vm.get_item_price(Item::Tea), 121);
        assert_eq!(vm.get_item_price(Item::Ham), 37);
        assert_eq!(vm.get_item_price(Item::Jupiter), 183);
    }

    #[test]
    fn test_vending_machine_buy() {
        let mut vm = VendingMachine::new(vec![Item::Coffee, Item::Tea, Item::Ham, Item::Jupiter]);
        vm.add_item(Item::Coffee, 5);
        let _ = vm.insert_coin(Coin::Euro);
        assert_eq!(vm.buy(Item::Coffee), Ok(53));
        assert_eq!(vm.buy(Item::Tea), Err("not enough money"));
        assert_eq!(vm.buy(Item::Ham), Ok(63));
        assert_eq!(vm.buy(Item::Jupiter), Err("not enough money"));
    }
}