#![allow(dead_code)]


// 1: Define a trait Printable that defines a method print that printlns self to the console.
// Implement this trait for i32 , String and Vec<T> where T implements Printable .
// Create a function print that takes a generic argument T that implements Printable and
// calls print on T .
// Decide whether to use monomorphization or dynamic dispatch for the print function.

use std::fmt::{Debug, Display};

trait Printable{
    fn printlns(&self);
}

impl Printable for i32{
    fn printlns(&self){
        println!("{}", self);
    }
}

impl Printable for String{
    fn printlns(&self){
        println!("{}", self);
    }
}

impl<T: Printable> Printable for Vec<T>{
    fn printlns(&self) {
        for x in self.into_iter(){
            x.printlns();
        }
    }
}

fn myprint<T: Printable>(x: T){
    x.printlns();
}


// 2: Define a struct Book with a title and a category (enum with Fantasy and Horror variants).
// Define a struct Library with a fixed size array of 10 bookcases, each containing a Vec of books.
// Implement a method populate for Library that fills each bookcase with 3 books.
// Implement a default method for Book

#[derive(Debug, Default)]
enum Category{
    #[default]
    Fantasy,
    Horror,
}

#[derive(Debug)]
struct Book<'a>{
    title: &'a str,
    cat: Category,
}

#[derive(Debug, Default)]
struct Library<'a>{
    bookcases: [Vec<Book<'a>>; 10],
}

impl<'a> Default for Book<'a>{
    fn default() -> Self {
        Book{title: "haha", cat: Category::Fantasy}
    }
}

impl<'a> Book<'a>{
    fn default_with_cat(c: Category) -> Self {
        Book {cat: c, ..Book::default()}
    }
}

trait Populatable{
    fn populate(&mut self);
}

impl<'a> Populatable for Library<'a>{
    fn populate(&mut self) {
        for f in self.bookcases.iter_mut(){
            f.push(Book::default());
            f.push(Book::default());
            f.push(Book::default());
        }
    }
}


// 3:
fn restricted<T, U>(t1: T, t2: T, u: U) -> T
where T: PartialOrd, T: Debug, U: Display{
    let smaller = if t1<t2 {t1} else {t2};
    println!("minor: {:?}", smaller);
    println!("u: {}", u);
    smaller
}


// 4:
use std::iter::Iterator;
#[derive(Debug)]
struct Task{
    name: String,
    priority: i32,
    done: bool
}

struct Tasks{
    tasks: Vec<Task>,
}

impl Iterator for Tasks{
    type Item = Task;
    fn next(&mut self) -> Option<Self::Item> {
        self.tasks.retain(|t| !t.done);
        self.tasks.iter().position(|t| !t.done).map(|t| self.tasks.remove(t))
    }
}


// 5: 
use std::ops::*;
struct Pair{
    pair: (i32, String)
}

impl Add<i32> for Pair{
    type Output = Pair;
    fn add(self, rhs: i32) -> Self::Output {
        Pair{pair: (self.pair.0 + rhs, self.pair.1)}
    }
}

impl Sub<i32> for Pair{
    type Output = Pair;
    fn sub(self, rhs: i32) -> Self::Output {
        Pair{pair: (self.pair.0 - rhs, self.pair.1)}
    }
}

impl Add<&str> for Pair{
    type Output = Pair;
    fn add(self, rhs: &str) -> Self::Output {
        let mut news = self.pair.1.clone();
        news.extend(rhs.chars());
        Pair{pair: (self.pair.0, news)}
    }
}

impl Sub<&str> for Pair{
    type Output = Pair;
    fn sub(self, rhs: &str) -> Self::Output {
        let news = self.pair.1.clone().replace(rhs, "");
        Pair{pair: (self.pair.0, news)}
    }
}

impl Add for Pair{
    type Output = Pair;
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.pair.0 + rhs.pair.1.as_str()
    }
}

impl Sub for Pair{
    type Output = Pair;
    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.pair.0 - rhs.pair.1.as_str()
    }
}

impl Mul<i32> for Pair {
    type Output = Pair;
    fn mul(self, rhs: i32) -> Self::Output {
        let mut news = String::new();
        for _ in 0..rhs{
            news = news.add(self.pair.1.as_str());
        }
        Pair{pair: (self.pair.0.pow(rhs as u32), news)}
    }
}


// 6: 
use rand::{self, Rng};
#[derive(Debug)]
struct Open;
#[derive(Debug)]
struct Closed;
#[derive(Debug)]
struct Stopped {
    _reason: String,
}
#[derive(Debug)]
struct Gate<S> {
    _state: S,
}

impl Gate<Open> {
    pub fn new() -> Gate<Open> {
        Gate { _state: Open }
    }
    pub fn close(self) -> Result<Gate<Closed>, Gate<Stopped>> {
        match rand::thread_rng().gen_range(0..20) {
            0..=12 =>  Ok(Gate { _state: Closed }),
            13..=15 => Err(Gate { _state: Stopped { _reason: "Motor error".to_string()}}),
            _ => Err(Gate {_state: Stopped {_reason: "Photocell detected an object".to_string()}})
        }
    }
}

impl Gate<Stopped> {
    pub fn new(reason: &str) -> Gate<Stopped> {
        Gate {
            _state: Stopped {
                _reason: reason.to_string(),
            },
        }
    }
    pub fn open(self) -> Gate<Open> {
        Gate { _state: Open }
    }
    pub fn close(self) -> Gate<Closed> {
        Gate { _state: Closed }
    }
}

impl Gate<Closed> {
    pub fn new() -> Gate<Closed> {
        Gate { _state: Closed }
    }
    pub fn open(self) -> Result<Gate<Open>, Gate<Stopped>> {
        match rand::thread_rng().gen_range(0..20) {
            0..=12 => Ok(Gate { _state: Open }),
            13..=15 => Err(Gate::<Stopped>::new("Motor error")),
            _ => Err(Gate::<Stopped>::new("Photocell detected an object")),
        }
    }
}


// 7: 
trait Heatable{
    fn cook(&mut self);
}

trait Friable{
    fn cook(&mut self);
}

trait Heater{
    fn heat(&self, h :&mut dyn Heatable){
        h.cook();
    }
}

trait Frier{
    fn fry(&self, h :&mut dyn Friable){
        h.cook();
    }
}

struct Oven{ }
struct Pan{ }

impl Heater for Oven{ }
impl Heater for Pan{ }
impl Frier for Pan{ }

#[derive(PartialEq)]
enum CarrotState{
    Raw, Cooked, Fried, Burnt
}
struct Pie{ ready: bool }
struct Carrot{ state: CarrotState }

trait Edible{
    fn eat(&self);
}

impl Heatable for Pie{
    fn cook(&mut self) {
        if self.ready{
            println!("You burned the pie")
        } else {
            self.ready = true;
        }
    }
}

impl Heatable for Carrot{
    fn cook(&mut self) {
        if self.state==CarrotState::Raw{
            self.state = CarrotState::Cooked;
        } else {
            self.state = CarrotState::Burnt;
        }
    }
}

impl Friable for Carrot{
    fn cook(&mut self) {
        if self.state==CarrotState::Raw{
            self.state = CarrotState::Fried;
        } else {
            self.state = CarrotState::Burnt;
        }
    }
}

impl Edible for Pie{
    fn eat(&self) {
        if self.ready{
            println!("yummy")
        } else {
            println!("you got stomachache")
        }
    }
}

impl Edible for Carrot{
    fn eat(&self) {
        match self.state {
            CarrotState::Raw => println!("mmh, crunchy"),
            CarrotState::Cooked => println!("mmh, yummy"),
            CarrotState::Fried => println!("mmh, crispy"),
            CarrotState::Burnt => println!("mmh, burnt")
        }
    }
}

fn main() {

}
