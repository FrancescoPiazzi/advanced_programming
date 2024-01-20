#![allow(dead_code)]

// 11:45 -> 12:50

use std::collections::HashMap;

// 1: Write a function called modify_odd that takes a mutable reference to an array slice of
// integers slice and sets all odd numbers to 0.
// Then write a second function that create a Vec, filled with all numbers from 0 to 100,
// and pass it to modify_odd ;
fn modify_odd(a: &mut[i32]){
    for (i, x) in a.iter_mut().enumerate(){
        if i%2==1{
            *x = 0;
        }
    }
}
fn create_vec() -> Vec<i32>{
    let mut v: Vec<i32> = Vec::new();
    for i in 0..=100{
        v.push(i);
    }
    modify_odd(v.as_mut_slice());
    v
}


// 2: Write a function count_character that takes a string consisting of ASCII characters
// string as input and returns a HashMap. The keys of the HashMap should be the
// characters in the string, and the values should be an u32 representing how many times
// each character appears in the string.
fn count_character(s: String) -> HashMap<char, u32> {
    let mut hm = HashMap::new();
    for c in s.chars(){
        hm.entry(c).and_modify(|x| *x+=1).or_insert(1);
    }
    hm
}


// 3: Write a function named split_at_value that takes two arguments: a slice of i32 called
// slice and a single i32 value called value . The function should find the first element
// equal to value inside slice . It should then split the slice at the corresponding index
// and return the two resulting slices wrapped in an Option.
// If value is not found in slice , the function should return None
fn split_at_value(slice: &[i32], value: i32) -> Option<(&[i32], &[i32])>{
    let mut pos = 0;
    let mut found = false;
    for (i, val) in slice.iter().enumerate(){
        if *val == value{
            found = true;
            pos = i
        }
    }
    if found {
        Some((&slice[0..pos], &slice[pos+1..slice.len()]))
    } else {
        None
    }
}


// 4: Write a function sub_slice that takes two &Vec<i32> as input. If the second vector is
// contained inside the first one it print the corresponding slice, otherwise it print Not
// found ;
fn sub_slice(v1: &Vec<i32>, v2: &Vec<i32>){
    let v1len = v1.len();
    let v2len = v2.len();
    if v1len > v2len {
        println!("Not found");
    } else {
        let mut i1=0;
        let mut i2=0;
        while i1 < v1len && i2 < v2len{
            if v1[i1] == v2[i2]{
                i2 += 1;
            } else {
                i2 = 0;   
            }
            i1 += 1;
        }
        if i2 == v2len{
            println!("{:?}", v2.as_slice());
        } else {
            println!("Not found");
        }
    }
}


// 5: Write the following functions, for each of the functions think carefully about what is the
// best way to pass the arguments (&, &mut or passing ownership):
// Write a function max that takes a Vec of i32 and returns the maximum value inside it.
// Write a function swap that swaps the first and last element of a vector of i32.
// Write a function is_sorted that takes a Vec of i32 and returns a boolean indicating
// whether the vector is sorted in non-decreasing order.
// Write a function insert_if_longer that takes a Vec of String ( vec ) and a String
// ( string ). This function should insert string into vec only if the length of string
// is greater than 10.
fn max(v: &Vec<i32>) -> i32{
    *v.iter().max().unwrap_or(&0)
}
fn swap(v: &mut Vec<i32>){
    let size = v.len();
    let tmp = v[0];
    v[0] = v[size-1];
    v[size-1] = tmp;
}
fn is_sorted(v: &Vec<i32>) -> bool{
    let mut prevx = v[0];
    for i in 1..v.len(){
        if v[i] >= prevx{
            prevx = v[i];
        } else {
            return false;
        }
    }
    true
}
fn insert_if_longer(v: &mut Vec<String>, s: String){
    if s.len() > 10{
        v.push(s);
    }
}


// 6:  Write a function build_vector that takes a Iter<i32> and returns the Vec<&i32>
// containing all the elements of the iterator;
use core::slice::Iter;
fn build_vector(iter: Iter<i32>) -> Vec<&i32>{
    iter.collect()
}

// 7: Write a function pancake_sort that takes a &mut Vec<i32> and sorts it using the
// pancake sort algorithm;
fn flip(arr: &mut Vec<i32>, mut k: i32){
    let mut left = 0;
    while left < k{
        arr.swap(left as usize, k as usize);
        k -= 1;
        left += 1
    }
}
fn max_index(arr: &mut Vec<i32>, k: i32) -> usize{
    let mut index = 0;
    for i in 0..k{
        if arr[i as usize] > arr[index]{
            index = i as usize;
        }
    }
    return index
}
fn pancake_sort(arr: &mut Vec<i32>){
    let mut n = arr.len();
    while n > 1{
        let maxdex = max_index(arr, n as i32);
        if maxdex != n - 1{
                if maxdex != 0{
                    flip(arr, maxdex as i32);
                }
                flip(arr, (n-1) as i32);
            }
        n -= 1;
    }
}

// 8: Write a function merge that takes two &[i32] and returns a Vec<i32> . The returned
// vector should contain the elements of the two passed elements sorted, you can assume
// that the passed slices are sorted;
fn merge(s1: &[i32], s2: &[i32]) -> Vec<i32> {
    let mut v = Vec::new();
    let mut i1=0;
    let mut i2=0;
    while i1<s1.len() && i2<s2.len(){
        if s1[i1] < s2[i2]{
            v.push(s1[i1]);
            i1 += 1;
        } else {
            v.push(s2[i2]);
            i2 += 1;
        }
    }
    if i1 < s1.len(){
        while i1 < s1.len(){
            v.push(s1[i1]);
            i1+=1;
        }
    }
    if i2 < s2.len(){
        while i2 < s2.len(){
            v.push(s2[i2]);
            i2+=1;
        }
    }
    v
}


// 9: Create a Vec that can contain both an i32 and a String ;
enum Data{
    I(i32),
    S(String)
}
const VSTR: Vec<Data> = Vec::new();


// // 10: Write these enums to represent a mathematical expression:
// One enum is called Operation and can be: Add , Sub , Mul , Div .
// One enum is called Expression an can be:
// Number (contain inside an i32)
// Operation (contain inside a left Expression, a right Expression and an
// Operation)
// Note: the left and right expression must be wrapped around a Box
// Write a function evaluate_expression that take as input an Expression, and return a
// Result with a i32 if the result is evaluated correctly, or a string if an error occurs.
enum Operation{
    Add,
    Sub,
    Mul,
    Div
}
enum Expression{
    Number(i32),
    Operation(Box<Expression>, Operation, Box<Expression>)
}
fn evaluate_expression(e: Expression) -> Result<i32, String> {
    match e {
        Expression::Number(x) => Ok(x),
        Expression::Operation(exp1, op, exp2) => {
            let val1 = evaluate_expression(*exp1)?;
            let val2 = evaluate_expression(*exp2)?;
            match op {
                Operation::Add => Ok(val1 + val2),
                Operation::Sub => Ok(val1 - val2),
                Operation::Mul => Ok(val1 * val2),
                Operation::Div => if val2 != 0 {Ok(val1 / val2)} else {Err("Cannot divide by zero".to_string())},
            }
        }
    }
}


fn main() {
}


#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_modify_odd() {
        let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        modify_odd(&mut array);
        assert_eq!(array, [1, 0, 3, 0, 5, 0, 7, 0, 9, 0]);
    }

    #[test]
    fn test_create_vec() {
        let v = create_vec();
        assert_eq!(v.len(), 101);
        for (i, &item) in v.iter().enumerate() {
            if i % 2 == 1 {
                assert_eq!(item, 0);
            } else {
                assert_eq!(item, i as i32);
            }
        }
    }

    #[test]
    fn test_count_character() {
        let s = "Hello World!".to_string();
        let hm = count_character(s);
        assert_eq!(hm.get(&'H'), Some(&1));
        assert_eq!(hm.get(&'e'), Some(&1));
        assert_eq!(hm.get(&'l'), Some(&3));
        assert_eq!(hm.get(&'o'), Some(&2));
        assert_eq!(hm.get(&' '), Some(&1));
        assert_eq!(hm.get(&'W'), Some(&1));
        assert_eq!(hm.get(&'r'), Some(&1));
        assert_eq!(hm.get(&'d'), Some(&1));
        assert_eq!(hm.get(&'!'), Some(&1));
    }

    #[test]
    fn test_split_at_value() {
        let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let (left, right) = split_at_value(&slice, 5).unwrap();
        assert_eq!(left, [1, 2, 3, 4]);
        assert_eq!(right, [6, 7, 8, 9]);

        let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let res = split_at_value(&slice, 10);
        assert_eq!(res, None);

        let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let (left, right) = split_at_value(&slice, 1).unwrap();
        assert_eq!(left, []);
        assert_eq!(right, [2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_sub_slice() {
        let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let v2 = vec![3, 4, 5];
        sub_slice(&v1, &v2);
        let v2 = vec![1, 2, 3];
        sub_slice(&v1, &v2);
        let v2 = vec![7, 8, 9];
        sub_slice(&v1, &v2);
        let v2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        sub_slice(&v1, &v2);
    }

    #[test]
    fn test_max() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(max(&v), 9);
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, -1];
        assert_eq!(max(&v), 9);
        let v = vec![-1, -2, -3, -4, -5, -6, -7, -8, -9];
        assert_eq!(max(&v), -1);
    }

    #[test]
    fn test_swap() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        swap(&mut v);
        assert_eq!(v, [9, 2, 3, 4, 5, 6, 7, 8, 1]);
        let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8];
        swap(&mut v);
        assert_eq!(v, [8, 2, 3, 4, 5, 6, 7, 1]);
    }

    #[test]
    fn test_is_sorted() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(is_sorted(&v), true);
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, -1];
        assert_eq!(is_sorted(&v), false);
        let v = vec![-1, -2, -3, -4, -5, -6, -7, -8, -9];
        assert_eq!(is_sorted(&v), false);
    }

    #[test]
    fn test_insert_if_longer() {
        let mut v = vec!["Hello".to_string(), "World".to_string()];
        insert_if_longer(&mut v, "Hello World!".to_string());
        assert_eq!(v, ["Hello".to_string(), "World".to_string(), "Hello World!".to_string()]);
        insert_if_longer(&mut v, "Hello".to_string());
        assert_eq!(v, ["Hello".to_string(), "World".to_string(), "Hello World!".to_string()]);
    }

    #[test]
    fn test_build_vector() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let v2 = build_vector(v.iter());
        assert_eq!(v2, [&1, &2, &3, &4, &5, &6, &7, &8, &9]);
    }

    #[test]
    fn test_pancake_sort() {
        let mut v = vec![3, 4, 7, 8, 5, 6, 1, 2, 9];
        pancake_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        pancake_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, -1];
        pancake_sort(&mut v);
        assert_eq!(v, [-1, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_merge() {
        let v1 = vec![1, 3, 5, 7, 9];
        let v2 = vec![2, 4, 6, 8, 10];
        assert_eq!(merge(&v1, &v2), [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let v1 = vec![1, 3, 5, 7, 9];
        let v2 = vec![2, 4, 6, 8];
        assert_eq!(merge(&v1, &v2), [1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let v1 = vec![1, 3, 5, 7];
        let v2 = vec![2, 4, 6, 8, 10];
        assert_eq!(merge(&v1, &v2), [1, 2, 3, 4, 5, 6, 7, 8, 10]);
    }

    #[test]
    fn test_evaluate_expression() {
        let expr = Expression::Operation(
            Box::new(Expression::Number(5)),
            Operation::Add,
            Box::new(Expression::Number(3))
        );
        assert_eq!(evaluate_expression(expr), Ok(8));

        let expr = Expression::Operation(
            Box::new(Expression::Number(10)),
            Operation::Sub,
            Box::new(Expression::Number(3))
        );
        assert_eq!(evaluate_expression(expr), Ok(7));

        let expr = Expression::Operation(
            Box::new(Expression::Number(4)),
            Operation::Mul,
            Box::new(Expression::Number(3))
        );
        assert_eq!(evaluate_expression(expr), Ok(12));

        let expr = Expression::Operation(
            Box::new(Expression::Number(9)),
            Operation::Div,
            Box::new(Expression::Number(3))
        );
        assert_eq!(evaluate_expression(expr), Ok(3));

        let expr = Expression::Operation(
            Box::new(Expression::Number(9)),
            Operation::Div,
            Box::new(Expression::Number(0))
        );
        assert_eq!(evaluate_expression(expr), Err("Cannot divide by zero".to_string()));
    }
}