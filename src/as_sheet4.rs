#![allow(dead_code)]


// 1 Write a function find_equal, that takes two &str as input s1 and s2 . The function
// look for a string slice of length 2 that appear in both s1 and s2 . If successful the
// function returns a tuple with the two equal slices. one from s1 , the other one from s2 .
// otherwise it return None
// Then write a second function lucky_slice , that take a n &str named input_str as
// input. The function create a String with the same length as input_str , filled with
// random lowercase letters, and then call find_equal on the two strings. If find_equal is
// successful the function lucky_slice return the slice of input_str that was found by
// find_equal . otherwise it returns None
fn find_equal<'a,'b>(s1: &'a str, s2: &'b str) -> Option<(&'a str, &'b str)> {
    for i in 0..=s1.len()-2{
        let slice1 = &s1[i..i+2];
        for j in 0..=s2.len()-2{
            let slice2 = &s2[j..j+2];
            println!("slice1: {}, slice2: {}", slice1, slice2);
            if slice1==slice2{
                return Some((slice1, slice2))
            }
        }
    }
    None
}

use rand::Rng;
fn lucky_slice<'a>(input_str: &'a str) -> Option<&'a str>{
    let mut rng = rand::thread_rng();
    let mut rand_s = String::new();
    for _ in 0..input_str.len(){
        rand_s.push(rng.gen_range('a'..='z') as char);
    }
    match find_equal(input_str, rand_s.as_str()){
        Some(res) => Some(res.0),
        None => None
    }
}


// 2: Write a struct Person that has 3 fields. a name with type String, and two parents (father and mother). 
// Each parent is an Option (lol) of an immutable reference to a Person. Then implement the following methods:
// a new method, that takes a name and two options to the parents and then returns a new Person.
// a find_relatives method that take a u32 input named generations. the function
// returns a Vec containing all the relatives within the generations range... for example:
// if generations is 0, the return should be just the person itself
// if generations is 1, the return should be the person itself + his parents
// if generations is 2, the function should return person itself + his parents + his grandparents.
// a find_roots method that returns a Vec containing all the relatives that has at
// least one parent set to None. Implement the second and the third methods recursively
use std::fmt::{Debug, Formatter};

#[derive(Clone, PartialEq)]
struct Person{
    name: String,
    parents: (Box<Option<Person>>, Box<Option<Person>>)
}

impl Debug for Person{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Person{
    fn new(s: String, p1: Option<Person>, p2: Option<Person>) -> Person{
        Person{name: s, parents: (Box::new(p1), Box::new(p2))}
    }

    fn find_relatives(&self, generations: u32) -> Vec<Person>{
        if generations==0{
            return vec![self.clone()];
        }
        let mut res: Vec<Person> = Vec::new();
        if let Some(p0) = *self.parents.0.clone() {
            res.append(&mut p0.find_relatives(generations-1));
        }
        if let Some(p1) = *self.parents.1.clone() {
            res.append(&mut p1.find_relatives(generations-1));
        }
        res.append(&mut vec![self.clone()]);
        res
    }

    fn is_orphan(&self) -> bool{
        self.parents.0.is_none() || self.parents.1.is_none()
    }

    fn find_roots(&self) -> Vec<Person> {
        let mut res: Vec<Person> = Vec::new();
        if self.is_orphan(){
            res.push(self.clone());
        }
        if let Some(p0) = *self.parents.0.clone() {
            res.append(&mut p0.find_roots());
        }
        if let Some(p1) = *self.parents.1.clone() {
            res.append(&mut p1.find_roots());
        }
        res
    }
}


// 3: Correct the following code
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a: 'b, 'b> ImportantExcerpt<'a> { //THIS LINE
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}


// 4: Annotate struct with lifetime: r and s must have different lifetimes
// lifetime of s is bigger than that of r
struct DoubleRef<'a: 'b, 'b, T> {
    r: &'b T,
    s: &'a T
}


// 5: Split Trait
// Write a trait split that has one generic type ReturnType. The trait has one method split that takes
// an immutable reference to self and return a tuple (ReturnType,ReturnType). The function should split the
// element of the trait in half. Implement the trait for:
// String , where split returns (&str, &str)
// &[i32] , where split returns (&[i32], &[i32])
// LinkedList<f64> , where split returns (LinkedList<f64>, LinkedList<f64>)
use std::collections::{LinkedList, HashMap};

trait Split<'a>{
    type ReturnType;
    fn split<ReturnType>(&'a self) -> (Self::ReturnType, Self::ReturnType);
}

impl<'a> Split<'a> for String{
    type ReturnType = &'a str;
    fn split<ReturnType>(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        self.as_str().split_at(self.len()/2)
    }
}

impl<'a> Split<'a> for &[i32]{
    type ReturnType = &'a [i32];
    fn split<ReturnType>(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        self.split_at(self.len()/2)
    }
}

impl<'a> Split<'a> for LinkedList<f64>{
    type ReturnType = Self;
    fn split<ReturnType>(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        let mut first = self.clone();
        let second = first.split_off(self.len()/2);
        (first, second)
    }
}


// 6: Geometry (all the stuff before ex 7)
use std::default::Default;
use std::fmt::Display;
use core::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
struct Point{
    x: f64,
    y: f64,
}

#[derive(Debug, PartialEq)]
struct Circle{
    center: Point,
    radius: f64,
}

#[derive(Debug, PartialEq)]
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

impl Default for Point{
    fn default() -> Self {
        Point{x:0.0, y:0.0}
    }
}

impl Default for Circle{
    fn default() -> Self {
        Circle{center: Point{x:0.0, y:0.0}, radius: 0.0}    // Circle{center: default(), radius: 0.0}
    }
}

impl Default for Rectangle{
    fn default() -> Self {
        Rectangle{top_left: Point{x:-1.0, y:1.0}, bottom_right: Point{x:1.0, y:-1.0}}
    }
}

impl Add for Rectangle{
    type Output = Rectangle;
    fn add(self, rhs: Self) -> Self::Output {
        Rectangle{
            top_left: Point{x: self.top_left.x+rhs.top_left.x, y: self.top_left.y+rhs.top_left.y}, 
            bottom_right: Point{x: self.bottom_right.x+rhs.bottom_right.x, y: self.bottom_right.y+rhs.bottom_right.y}
        }
    }
}

impl Sub for Rectangle{
    type Output = Rectangle;
    fn sub(self, rhs: Self) -> Self::Output {
        Rectangle{
            top_left: Point{x: self.top_left.x-rhs.top_left.x, y: self.top_left.y-rhs.top_left.y}, 
            bottom_right: Point{x: self.bottom_right.x-rhs.bottom_right.x, y: self.bottom_right.y-rhs.bottom_right.y}
        }
    }
}

#[derive(Debug, PartialEq)]
struct  Area{
    area: f64,
}

impl Default for Area{
    fn default() -> Self {
        Area{area: 0.0}
    }
}

impl Display for Area{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Area is {} cm²", self.area)
    }
}

trait GetArea{
    fn get_area(&self) -> Area;
}

impl GetArea for Point{
    fn get_area(&self) -> Area {
        Area{area: 0.0}
    }
}

impl GetArea for Circle{
    fn get_area(&self) -> Area {
        Area{area: 3.1415926*self.radius*self.radius}
    }
}

impl GetArea for Rectangle{
    fn get_area(&self) -> Area {
        Area{area: ((self.bottom_right.x-self.top_left.x)*(self.top_left.y-self.bottom_right.y)).abs()}
    }
}

impl Add for Area{
    type Output = Area;
    fn add(self, rhs: Self) -> Self::Output {
        Area{area: self.area+rhs.area}
    }
}

impl Add for &dyn GetArea{
    type Output = Area;
    fn add(self, rhs: Self) -> Self::Output {
        self.get_area()+rhs.get_area()
    }
}

fn sum_area(slice: &[&dyn GetArea]) -> Area{
    let mut sum = Area::default();
    for a in slice{
        sum = sum + a.get_area();
    }
    sum
}


// 7: Create a function 'skip_prefix' that, given non mutable references to telephone_number:
// &str and prefix: &str , returns &str . The function removes the prefix from the
// number, and if there isn't the prefix contained at the start of number , return number itself.
fn skip_prefix<'a>(telephone_number: &'a str, prefix: &str) -> &'a str{    
    if telephone_number.starts_with(prefix){
        &telephone_number[prefix.len()..]
    } else {
        telephone_number
    }
}


// 8: Create a struct Chair that has two fields: color: &str and quantity: &usize . Create
// another structure Wardrobe that has the same fields of Chair . Create a trait Object
// that has two function declarations: build that has &self as argument and returns a
// &str ; get_quantity that has &self as argument and return a String .
// Then, implement the trait Object for Chair and Wardrobe
struct Chair<'a>{
    color: &'a str,
    quantity: &'a usize,
}

struct Wardrobe<'a>{
    color: &'a str,
    quantity: &'a usize,
}

trait BuildAndCount{
    fn build(&self) -> &str;
    fn get_quantity(&self) -> String;
}

impl<'a> BuildAndCount for Chair<'a>{
    fn build(&self) -> &str {
        "Chair has been built"
    }

    fn get_quantity(&self) -> String {
        let mut res = String::new();
        res.push_str("There ");
        if *self.quantity == 1 {
            res.push_str("is 1 chair");
        } else {
            res.push_str("are ");
            res.push_str(&self.quantity.to_string());
            res.push_str(" chairs");
        }

        res
    }
}


impl<'a> BuildAndCount for Wardrobe<'a>{
    fn build(&self) -> &str {
        "Wardrobe has been built"
    }

    fn get_quantity(&self) -> String {
        "There are ".to_string() + &self.quantity.to_string() + " wardrobes"
    }
}


// 9: description is longer than the code
#[derive(PartialEq, Eq, PartialOrd)]
enum Role{
    GUEST,
    USER,
    ADMIN,
}

#[derive(PartialEq, Eq, Hash)]
enum Permission{
    READ,
    WRITE,
    EXECUTE,
}

struct Actions{
    action: String,
    permission: HashMap<Permission, bool>,
}

struct User{
    name: String,
    role: Role,
    actions: Vec<Actions>,
}


trait Auth{
    fn check_permissions(&self, action: &str, permission_type: Permission) -> bool;
    fn can_read(&self, string: &str) -> bool;
    fn can_write(&self, string: &str) -> bool;
    fn can_execute(&self, string: &str) -> bool;
}

impl Auth for User{
    fn check_permissions(&self, action: &str, permission_type: Permission) -> bool {
        for a in &self.actions{
            if a.action == action{
                match a.permission.get(&permission_type) {
                    Some(p) => return *p,
                    None => return false,
                }
            }
        }
        false
    }

    fn can_read(&self, action: &str) -> bool {
        self.check_permissions(action, Permission::READ)
    }

    fn can_write(&self, action: &str) -> bool {
        self.check_permissions(action, Permission::WRITE)
    }

    fn can_execute(&self, action: &str) -> bool {
        self.check_permissions(action, Permission::EXECUTE)
    }
}

impl Actions{
    fn new(action: String, read: bool, write: bool, execute: bool) -> Actions{
        let mut hm: HashMap<Permission, bool> = HashMap::new();
        hm.insert(Permission::READ, read);
        hm.insert(Permission::WRITE, write);
        hm.insert(Permission::EXECUTE, execute);
        Actions{
            action: action,
            permission: hm,
        }
    }
}

impl Default for User{
    fn default() -> Self {
        User{name: "Guest".to_string(), role: Role::GUEST, actions: Vec::new()}
    }
}

impl User{
    fn change_role(&mut self, role: Role) -> Result<(), String>{
        if self.role >= role{
            self.role = role;
            Ok(())
        } else {
            Err("Permission not valid".to_string())
        }
    }
}

fn sudo_change_permission(mut user: User, string: String, permission: Permission, value: bool){
    for a in user.actions.iter_mut(){
        if a.action == string{
            a.permission.insert(permission, value);
            break;
        }
    }
}


fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_equal() {
        assert_eq!(find_equal("hello", "hel"), Some(("he", "he")));
        assert_eq!(find_equal("hllo", "hell"), Some(("ll", "ll")));
    }

    #[test]
    fn test_lucky_slice() {
        lucky_slice("hello");
    }

    #[test]
    fn test_find_relatives() {
        let p1 = Person::new("p1".to_string(), None, None);
        let p2 = Person::new("p2".to_string(), None, None);
        let p3 = Person::new("p3".to_string(), Some(p1.clone()), Some(p2.clone()));
        let p4 = Person::new("p4".to_string(), Some(p3.clone()), None);
        let p5 = Person::new("p5".to_string(), None, None);
        let p6 = Person::new("p6".to_string(), Some(p4.clone()), Some(p5.clone()));
        let p7 = Person::new("p7".to_string(), Some(p6.clone()), None);

        assert_eq!(p7.find_relatives(0), vec![p7.clone()]);
        assert_eq!(p7.find_relatives(2), vec![p4.clone(), p5.clone(), p6.clone(), p7.clone()]);
    }

    #[test]
    fn test_find_roots() {
        let p1 = Person::new("p1".to_string(), None, None);
        let p2 = Person::new("p2".to_string(), None, None);
        let p3 = Person::new("p3".to_string(), Some(p1.clone()), Some(p2.clone()));
        let p4 = Person::new("p4".to_string(), Some(p3.clone()), None);
        let p5 = Person::new("p5".to_string(), None, None);
        let p6 = Person::new("p6".to_string(), Some(p4.clone()), Some(p5.clone()));
        let p7 = Person::new("p7".to_string(), Some(p6.clone()), None);

        assert_eq!(p7.find_roots(), vec![p7, p4, p1, p2, p5]);
    }

    // test point 6: Geometry
    #[test]
    fn test_default() {
        let p = Point::default();
        assert_eq!(p, Point{x:0.0, y:0.0});
        let c = Circle::default();
        assert_eq!(c, Circle{center: Point{x:0.0, y:0.0}, radius: 0.0});
        let r = Rectangle::default();
        assert_eq!(r, Rectangle{top_left: Point{x:-1.0, y:1.0}, bottom_right: Point{x:1.0, y:-1.0}});
    }

    #[test]
    fn test_add() {
        let r1 = Rectangle{top_left: Point{x:0.0, y:0.0}, bottom_right: Point{x:1.0, y:1.0}};
        let r2 = Rectangle{top_left: Point{x:1.0, y:1.0}, bottom_right: Point{x:2.0, y:2.0}};
        assert_eq!(r1+r2, Rectangle{top_left: Point{x:1.0, y:1.0}, bottom_right: Point{x:3.0, y:3.0}});
    }

    #[test]
    fn test_sub() {
        let r1 = Rectangle{top_left: Point{x:0.0, y:0.0}, bottom_right: Point{x:1.0, y:1.0}};
        let r2 = Rectangle{top_left: Point{x:1.0, y:1.0}, bottom_right: Point{x:2.0, y:2.0}};
        assert_eq!(r1-r2, Rectangle{top_left: Point{x:-1.0, y:-1.0}, bottom_right: Point{x:-1.0, y:-1.0}});
    }

    #[test]
    fn test_get_area() {
        let p = Point{x:0.0, y:0.0};
        assert_eq!(p.get_area(), Area{area: 0.0});
        let c = Circle{center: Point{x:0.0, y:0.0}, radius: 1.0};
        assert_eq!(c.get_area(), Area{area: 3.1415926});
        let r = Rectangle{top_left: Point{x:0.0, y:0.0}, bottom_right: Point{x:1.0, y:1.0}};
        assert_eq!(r.get_area(), Area{area: 1.0});
    }

    #[test]
    fn test_sum_area() {
        let p = Point{x:0.0, y:0.0};
        let c = Circle{center: Point{x:0.0, y:0.0}, radius: 1.0};
        let r = Rectangle{top_left: Point{x:0.0, y:0.0}, bottom_right: Point{x:1.0, y:1.0}};
        let slice: &[&dyn GetArea] = &[&p, &c, &r];
        assert_eq!(sum_area(slice), Area{area: 4.1415926});
    }

    #[test]
    fn test_skip_prefix() {
        assert_eq!(skip_prefix("123456789", "123"), "456789");
        assert_eq!(skip_prefix("123456789", "456"), "123456789");
    }

}