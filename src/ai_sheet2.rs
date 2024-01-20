#![allow(dead_code)]

use std::collections::HashMap;

fn modify_odd(v: &mut [i32]) {
    let mut i=0;
    while i < v.len(){
        if i%2 == 1 {
            v[i] = 0;
        }
        i += 1;
    }
}

fn generate_vec() -> Vec<i32> {
    let mut v = Vec::new();
    let mut i=0;
    while i<101 {
        v.push(i);
        i+=1;
    }
    v
}

fn count_character(s: String) -> HashMap<char, i32> {
    let mut hm: HashMap<char, i32> = HashMap::new();
    for c in s.chars(){
        match hm.get_mut(&c){
            Some(x) => {
                *x += 1;
            }
            None => {
                hm.insert(c,1);
            }
        }
    }
    hm
}

fn split_at_value(slice: &[i32], value: i32) -> Option<(&[i32], &[i32])>{
    let mut i=0;
    for x in slice{
        if *x==value{
            return Option::Some((&slice[..i], &slice[i+1..]));
        }
        i += 1;
    }
    return Option::None;
}

fn sub_slice(v1: &Vec<i32>, v2: &Vec<i32>) {
    let mut count = 0;
    let mut count2 = 0;
    let mut found = false;
    for x1 in v1 {
        if *x1 == v2[count] {
            count += 1;
            if count == v2.len() {
                println!("{:?}", &v1[count2+1-v2.len()..count2+1]);   // dovrebbe essere uguale a v2
                found = true;
                break;
            }
        }
        else {
            count = 0;
        }
        count2 += 1;
    }
    if !found {
        println!("v2 non è in v1");
    }
}

fn max(v: &Vec<i32>, i: usize, mut maxisofar: usize) -> i32 {
    if i == v.len() { return v[maxisofar as usize] }
    if i == 0 { maxisofar = 0; }
    else if v[i] > v[maxisofar] { maxisofar = i; }
    return  max(v, i+1, maxisofar);
}

fn swap(v: &mut[i32]){
    let tmp = v[0];
    v[0] = v[v.len()-1];
    v[v.len()-1] = tmp;
}

fn is_sorted(v: &Vec<i32>, i: usize, last: i32) -> bool {
    if i == v.len() {
        return true;
    } else {
        if i==0 || v[i] >= last{
            return is_sorted(v, i+1, v[i]);
        } else {
            return false;
        }
    }
}

fn insert_if_longer(mut v: Vec<String>, s: String) -> Vec<String> {
    if s.len() > 10{
        v.push(s);
    }
    v
}

use core::slice::Iter;
fn build_vector(it: Iter<i32>) -> Vec<&i32> {
    let mut res: Vec<&i32> = Vec::new();
    for i in it{
        res.push(i);
    }
    res
}

fn pancacke_sort(v: &mut Vec<i32>) -> &mut Vec<i32>{
    for i in 0..v.len(){
        for j in 0..v.len()-i-1{
            if v[j] > v[j+1]{
                let tmp = v[j];
                v[j] = v[j+1];
                v[j+1] = tmp;
            }
        }
    }
    v
}

fn merge(a: &[i32], b: &[i32]) -> Vec<i32>{
    let mut res: Vec<i32> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    while i1 < a.len() && i2 < b.len(){
        if a[i1] < b[i2]{
            res.push(a[i1]);
            i1 += 1;
        } else {
            res.push(b[i2]);
            i2 += 1;
        }
    }

    while i1 < a.len(){
        res.push(a[i1]);
        i1 += 1;
    }

    while i2 < b.len(){
        res.push(b[i2]);
        i2 += 1;
    }

    res
}


enum IString {
    I32(i32),
    String(String),
}

const MY_VECTOR: Vec<IString> = Vec::new();


enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}


enum Expression {
    Number(i32),
    Operation(Box<Expression>, Operation, Box<Expression>),
}

impl Expression {
    
    fn evaluate_expression(&self) -> i32 {
        match self {
            Expression::Number(n) => *n,
            Expression::Operation(e1, op, e2) => {
                let n1 = e1.evaluate_expression();
                let n2 = e2.evaluate_expression();
                match op {
                    Operation::Add => n1 + n2,
                    Operation::Sub => n1 - n2,
                    Operation::Mul => n1 * n2,
                    Operation::Div => n1 / n2,
                }
            }
        }
    }
}


fn main() {
    println!("Hello, world!");
    let mut v = generate_vec().clone();
    modify_odd(&mut v[..]);
    println!("{:?}", v);

    let hm: HashMap<char, i32> = count_character("non mi piace questo linguaggio".to_string());
    for k in hm.keys(){
        println!("{}: {}", k, hm.get(k).unwrap());
    }

    match split_at_value(&[4, 5, 7, 9], 7){
        Some(x) => {
            println!("{:?}", x);
        }
        None => {
            println!("value not found");
        }
    }

    sub_slice(&Vec::from([3, 4, 5, 6, 7]), &Vec::from([4, 5]));

    println!("{}", max(&Vec::from([-3, 7, 2]), 0, 0));

    let mut v: [i32; 4] = [3, 4, 5, 6];
    swap(&mut v);
    println!("{:?}", v);

    println!("{}", is_sorted(&Vec::from([1, 2, 2, 3]), 0, 0));

    let v = Vec::from(["haha".to_string(), "hehe".to_string()]);
    println!("{:?}", insert_if_longer(v, "Bananaannananna".to_string()));

    println!("{:?}", build_vector([2, 4 , 6, 8].iter()));

    println!("{:?}", pancacke_sort(&mut Vec::from([3, 2, 1, 4, 5])));

    println!("{:?}", merge(&[1, 3, 5, 7], &[2, 4, 6]));


}
