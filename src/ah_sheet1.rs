#![allow(dead_code)]

use std::collections::HashMap;

fn string_reverse(s: &str) -> String {
    // s.chars().rev().collect() // versione facile
    let mut result = String::new();
    for c in s.chars(){
        result.insert(0, c);
    }
    return result;
}

fn bigger(x: i32, y:i32) -> i32{
    if x > y { x } else { y }
}

fn multipy(x: i32, y:f32, z:f64) -> f64{
    (x as f64) * (y as f64) * z
}

const C:f32 = 300000.0;
fn e_equals_mc_squared(m: f32) -> f32{
    m * (C*C) as f32
}

fn max_min(a: Vec<i32>) -> (i32, i32){
    let mut min = a[0];
    let mut max = a[0];
    for n in a{
        if n<min { min  = n; } 
        if n>max { max  = n; } 
    }
    return (min, max);
}

fn lord_farquaad(s: String) -> String{
    // str::replace(s, "e", "💥");   // versione facile
    let res: String = s.chars().map(|x| match x { 
        'e' => '💥',
        _ => x
    }).collect();
    return res;
}

fn hlookup(map: &HashMap<String, f32>, s: String) -> f32{
    // *(map.get(&s).unwrap_or(&-1.0))  // versione facile
    if map.contains_key(&s) {
        *(map.get(&s).unwrap())
    } else {
        -1.0
    }
}

fn append(mut s: String) -> String{
    s.push_str("foobar");
    s
}

fn is_armstrong(x: i32) -> bool{
    if x<0 {return false;}
    if x==0 {return true;}
    
    let mut n = x;
    let mut digits = 0;
    let mut n1 = 1;
    let mut sum = 0;

    while n1 <= n{
        digits+=1;
        n1*=10
    }

    while n>0 {
        sum += (n%10).pow(digits);
        n = n/10;
    }

    sum == x
}

fn transpose(m: ((i32, i32), (i32, i32))) -> ((i32, i32), (i32, i32)){
    let mut res = m.clone();
    res.0.1 = m.1.0;
    res.1.0 = m.0.1;

    return res
}

fn main() {
    println!("{}", string_reverse("Hello, world!"));
    println!("{}", bigger(2, 3));
    println!("{}", multipy(2, 3.1, 2.12241));
    println!("{}", e_equals_mc_squared(1.3));
    println!("{:?}", max_min(vec![-3, 1, 0, 5, 30]));
    println!("{}", lord_farquaad("yayyeyee".to_string()));

    let mut furniture: HashMap<String, f32> = HashMap::new();
    furniture.insert("table".to_string(), 20.5);
    println!("{}", hlookup(&furniture, "table".to_string()));
    println!("{}", hlookup(&furniture, "thing".to_string()));

    let s1 = String::from("hello");
    let s2 = append(s1.clone());
    println!("{} {}", s2, s1);

    for x in [3, 9, 10, 153, 154]{
        println!("{}: {} ", x, is_armstrong(x));
    }

    println!("{:?}", transpose(((1, 2), (3, 4))));
}
