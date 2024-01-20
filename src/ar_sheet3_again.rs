#![allow(dead_code)]

// 1: Given a number determine whether it is valid per the Luhn formula, creating the function is_it_luhn
// The Luhn algorithm is a simple checksum formula used to validate a variety of identification numbers, such as credit card numbers. Check
// if a given string is valid
// Strings of length 1 or less are not valid. Spaces are allowed in the input, but they should be stripped before checking. All other non-digit
// characters are disallowed.
// The first step is to double every second digit, starting from the right. 
// If doubling the number results in a number greater than 9 then subtract 9 from the product.
// If the sum is divisible by 10, then the number is valid.
fn is_it_luhn(s: String) -> bool{
    if s.len() <= 1{
        return false;
    }
    let mut s1: String = String::new();
    for c in s.chars(){
        match c{
            '0'..='9' => s1.push(c),
            ' ' => {},
            _ => {return false}
        }
    }
    
    let mut sum = 0;
    for (i, c) in s1.chars().rev().enumerate(){
        if i%2==1{
            let add =(c as u32 - '0' as u32)*2;
            sum += if add <= 9 {add} else {add-9};
        }
    }
    !(sum%10 == 0)
}


// 2: or the following examples, decide which of the composite data structures is better (enum or structs). Then implement them
// you are Rick, a car shop owner, and you have to choose the fuel of your car between Diesel, Gasoline, LPG, Methane and Electric
// you have to program the recognition of the IP version of a router. Remember that IPv4 is formatted with 4 group of 3 integer values
// (from 0 to 255), IPv6 is instead formatted with 8 groups of 4 hexadecimal (so no strings!) values.
// you have to track points in a 3-dimensional space, with the f64 values for each dimension
enum FuelType{
    Diesel,
    Gasoline,
    LPG,
    Methane,
    Electric
}

enum IP{
    Ipv4([u8; 4]),
    Ipv6([u32; 8])
}

struct Point{
    x: f64,
    y: f64,
    z: f64
}


// 3: In Trento there is an automated car park with a camera that recognises the number plate of the car. Your task is to associate the number
// plate with the owner of the car in order to track the price for each car owner. Create a main with an appropriate data structure already
// initialised with some data. Create a function recognise_owner that, given the data structures mentioned above and the number of car
// plate, returns an Option al value of the owner of the car
struct Owner{
    name: String,
    address: String,
    phone: String
}



fn main() {
    println!("Hello, world!");
}
