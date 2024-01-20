#![allow(dead_code)]

fn main() {
    let s1 = String::from("hello");
    let _s2 = s1; //x moved to y
    // println!("{}", s1); // error, `s1` doesn't own the value anymore.

    // we can avoid this by cloning the value of s2
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // this is ok as well because x is a prmitive type
    // so the compiler can allocate the space needed for y at compile time
    // to be more precise, the type i32 has the "copy" trait,
    // types with this trait will always be copied when assigned
    let x: i32 = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}