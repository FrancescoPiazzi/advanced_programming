#![allow(dead_code)]

// 11:10 -> 11:30


// 1: Write a function string_reverse that takes a &str as input and returns it, reversed as a String
fn string_reverse(s: &str) -> String{
    s.chars().rev().collect()
}


// 2: Write a function bigger that takes two i32 and returns the bigger number ( i32 ) without using 
// another function call and additional variables;
fn bigger(a: i32, b:i32) -> i32{
    if a > b {a} else {b}
}


// 3: Write a function multiply that takes an i32 , a f32 and a f64 and returns the multiplication of the
// three of them as a f64 value;
fn multiply(x: i32, y: f32, z: f64) -> f64{
    x as f64 * y as f64 * z
}


// 4: Write a function e_equals_mc_squared that takes as input a f32 representing the mass, and that
// uses a globally-defined constant containing the value of the speed of light in a vacuum (expressed in
// m/s). The function outputs the energy equivalent to the mass input;
const C: f32 = 300000.0;
fn e_equals_mc_squared(mass: f32) -> f32 {
    mass*C*C
}


// 5: Given a vector of i32 , create a function max_min that returns the maximum and the minimum value
// inside that vector;
fn max_min(v: Vec<i32>) -> (i32, i32){
    let mut max = v[0];
    let mut min = v[0];
    for x in v {
        if x > max{
            max = x;
        }
        if x < min{
            min = x;
        }
    }
    (max, min)
}


// 6: Write a function lord_farquaad that takes a String and outputs another String in which every
// character 'e' is substituted by the character '💥';
fn lord_farquaad(s: String) -> String{
    s.chars().map(|x| match x {
        'e' => '💥',
        _ => x
    }).collect()
}


// 7: In the main function initialize a HashMap<String, f32> called furniture that stores the pair
// String as key and f32 as value, where the String is the name of the furniture and the f32 is its
// price. Then write a function that borrows the HashMap , takes a furniture: String as input and
// returns the corresponding f32 . If there is no such furniture in the HashMap , return -1.0 
use std::collections::HashMap;
fn hashmap_lookup(hm: &HashMap<String, f32>, furniture: String) -> f32{
    *hm.get(&furniture).unwrap_or(&-1.0)
}


// 8: Write a function append that takes a String , appends the word "foobar" to it and returns it.
// Then write a main function in which you:
// - Declare a String initialized with some text.;
// - Pass the String to the function append ;
// - Print the original String and the one returned by append ;
// (do it in this order!)
fn append(s: String) -> String {
    s.chars().chain("foobar".chars()).collect()
}


// 9: An Armstrong number is a number that is the sum of its own digits each raised to the power of the
// number of digits.
// Write the function is_armstrong that determines whether a number is an Armstrong number;
fn is_armstrong(x: u32) -> bool{
    let mut newx = x;
    let digits = x.to_string().len() as u32;
    let mut sum = 0;
    while newx > 0{
        sum += (newx%10).pow(digits);
        newx = newx/10;
    }
    sum == x
}


// 10: Write a function that takes a 2x2 i32 "matrix" (2x2 tuple) as input, transposes and returns it.
fn transpose(m: ((i32, i32), (i32, i32))) -> ((i32, i32), (i32, i32)){
    ((m.0.0, m.1.0), (m.0.1, m.1.1))
}


fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_reverse() {
        assert_eq!(string_reverse("hello"), "olleh".to_string());
    }

    #[test]
    fn test_bigger() {
        assert_eq!(bigger(5, 3), 5);
        assert_eq!(bigger(3, 5), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3.0, 4.0), 24.0);
    }

    #[test]
    fn test_e_equals_mc_squared() {
        assert_eq!(e_equals_mc_squared(2.0), 2.0 * C * C);
    }

    #[test]
    fn test_max_min() {
        assert_eq!(max_min(vec![1, 2, 3, 4, 5]), (5, 1));
    }

    #[test]
    fn test_lord_farquaad() {
        assert_eq!(lord_farquaad("hello".to_string()), "h💥llo".to_string());
    }

    #[test]
    fn test_hashmap_lookup() {
        let mut hm = HashMap::new();
        hm.insert("chair".to_string(), 50.0);
        assert_eq!(hashmap_lookup(&hm, "chair".to_string()), 50.0);
        assert_eq!(hashmap_lookup(&hm, "table".to_string()), -1.0);
    }

    #[test]
    fn test_append() {
        assert_eq!(append("hello".to_string()), "hellofoobar".to_string());
    }

    #[test]
    fn test_is_armstrong() {
        assert_eq!(is_armstrong(153), true);
        assert_eq!(is_armstrong(123), false);
    }

    #[test]
    fn test_transpose() {
        assert_eq!(transpose(((1, 2), (3, 4))), ((1, 3), (2, 4)));
    }
}