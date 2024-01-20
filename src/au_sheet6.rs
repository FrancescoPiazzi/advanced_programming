#![allow(dead_code)]


use std::rc::Rc;
// 1: 
struct TreeNode<T> where T:PartialOrd + Clone{
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> where T:PartialOrd + Clone{
    fn new(t: T) -> TreeNode<T> where T:PartialOrd + Clone{
        TreeNode{val: t, left: None, right: None }
    }

    fn insert(&mut self, el: &T) where T:PartialOrd + Clone{
        let next = if self.val > *el { &mut self.left } else { &mut self.right };

        match next {
            Some(node) => (*node).insert(el),
            None => *next = Some(Box::new(TreeNode::new(el.clone()))),
        }
    }

    fn from_vec(vec: &Vec<T>) -> Self{
        if vec.len() == 0 {
            panic!("Empty vector");
        }
        let mut root = TreeNode::new(vec[0].clone());
        for i in 0..vec.len() {     // or x in vec.iter().skip(1)
            root.insert(&vec[i]);
        }
        root
    }
}


// 2: Create a struct CarDealer with a field that is a vector of Car .
// Create a struct User with a field that is an Option of Car .
// Implement the following methods for CarDealer :
// new that takes a vector of Car and returns a CarDealer
// add_car that takes a Car and adds it to the vector of Car
// print_cars that prints all the cars
// rent_user that takes a mutable reference to a User and a model: String, that
// identify the car, and assigns the car to the user and set the rent field to true. If the
// car is not found, print "Car not found".
// The car must be the same present in the vector of CarDealer and into the car field
// of the User.
// end_rental that takes a mutable reference to a User and set the rent field to false.
// If the user has no car, print "User has no car".
// Implement the new and default method for Car
// Implement the print_car method for User that prints the car if it is present, otherwise
// print "User has no car"
use std::cell::RefCell;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Car{
    model: String,
    rent: bool,
}

impl Car{
    fn new(model: String) -> Car{
        Car{model: model, rent: false}
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct User{
    car: Option<Rc<RefCell<Car>>>,
}

impl User{
    fn new() -> User{
        User{car: None}
    }

    fn print_car(&self){
        match &self.car{
            Some(car) => println!("{:?}", car),
            None => println!("User has no car"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct CarDealer{
    cars: Vec<Rc<RefCell<Car>>>,
}

impl CarDealer{
    fn new(cars: Vec<Car>) -> CarDealer{
        CarDealer{cars: cars.iter().map(|x| Rc::new(RefCell::new(x.clone()))).collect()}
    }

    fn add_car(&mut self, car: Car){
        self.cars.push(Rc::new(RefCell::new(car)));
    }

    fn print_cars(&self){
        for car in &self.cars{
            println!("{:?}", car);
        }
    }

    fn rent_user(&mut self, user: &mut User, model: String){
        let car_option = self.cars
            .iter()
            .find(|car| RefCell::borrow(&**car).model == model);

        match car_option {
            Some(car) => {
                (&**car).borrow_mut().rent = true;
                user.car = Some(Rc::clone(car));
            },
            None => println!("Car not found"),
        }
    }

    fn end_rental(&mut self, user: &mut User){
        match &mut user.car{
            Some(car) => {
                (&**car).borrow_mut().rent = false;
                user.car = None;
            },
            None => println!("User has no car"),
        }
    }
}

// 3: 
trait Sound{
    fn make_sound(&self);
}

struct FarmCell{
    element: Box<dyn Sound>,
    next: Option<Box<FarmCell>>,
}

struct Dog;
struct Cat;

impl Sound for Dog{
    fn make_sound(&self){
        println!("Woof");
    }
}

impl Sound for Cat{
    fn make_sound(&self){
        println!("Meow");
    }
}

impl FarmCell{
    fn new(element: Box<dyn Sound>) -> FarmCell{
        FarmCell{element: element, next: None}
    }

    fn insert(&mut self, element: Box<dyn Sound>){
        match &mut self.next{
            Some(next) => next.insert(element),
            None => self.next = Some(Box::new(FarmCell::new(element))),
        }
    }
}

impl Sound for FarmCell{
    fn make_sound(&self){
        self.element.make_sound();
        match &self.next{
            Some(next) => next.make_sound(),
            None => (),
        }
    }
}


// 4: create the struct PublicStreetlight with the fields id , on and burn_out : it represent a
// public light, with its id, if it is on or off and if it is burned out or not.
// Create the struct PublicIllumination with the field lights that is a vector of PublicStreetlight.
// Implement the methods new and default for PublicStreetlight and
// PublicIllumination . Then implement the Iterator trait for PublicIllumination that
// returns the burned out lights in order of permit the public operators to change them. The
// iterator must remove the burned out lights from the vector
struct PublicStreetlight{
    id: u32,
    on: bool,
    burn_out: bool,
}

struct PublicIllumination{
    lights: Vec<PublicStreetlight>
}

impl PublicStreetlight{
    fn new(id: u32) -> Self{
        PublicStreetlight{id: id, on: false, burn_out: false}
    }
}

impl PublicIllumination{
    fn new() -> Self{
        PublicIllumination{lights: Vec::new()}
    }
}

impl Iterator for PublicIllumination{
    type Item = PublicStreetlight;
    fn next(&mut self) -> Option<Self::Item> {
        let mut i = 0;
        while i < self.lights.len(){
            if self.lights[i].burn_out{
                return Some(self.lights.remove(i));
            }
            i += 1;
        }
        None
    }
}


// 5: 
use std::marker::PhantomData;
trait CompileTimeNode{
    type LeftType: CompileTimeNode;
    type RightType: CompileTimeNode;
    fn is_none() -> bool;
}

struct NullNode{}
struct Node<L: CompileTimeNode, R: CompileTimeNode>{
    left: PhantomData<L>,
    right: PhantomData<R>
}

impl<L,R> CompileTimeNode for Node<L,R>
where L: CompileTimeNode, R:CompileTimeNode{
    type LeftType = L;
    type RightType = R;

    fn is_none() -> bool {
        false
    }
}

impl CompileTimeNode for NullNode{
    type LeftType = NullNode;    // this makes no sense
    type RightType = NullNode;

    fn is_none() -> bool {
        true
    }
}

fn count_nodes<T: CompileTimeNode>() -> usize{
    if T::is_none(){
        0
    } else {
        1 + count_nodes::<T::LeftType>() + count_nodes::<T::LeftType>()
    }
}


// 7: Create a struct named EngangledBit .
// When two bits b1 and b2 are entangled with each-other they are connected,
// meanings that they will always have the same value.
// A bit can be entangled with any number of other bits (including 0)
struct EntangledBit{
    val: Rc<RefCell<bool>>,
}

impl Default for EntangledBit {
    fn default() -> Self {
        EntangledBit{val: Rc::new(RefCell::new(false))}
    }
}

impl EntangledBit{
    fn set(&mut self){
        *self.val.borrow_mut() = true;
    }

    fn reset(&mut self){
        *self.val.borrow_mut() = false;
    }

    fn get(&self) -> bool{
        *self.val.borrow()
    }

    fn entangle(&mut self, bit: &mut EntangledBit){
        bit.val = Rc::clone(&self.val);
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut root = TreeNode::new(5);
        let haha =&6;
        let hehe =&2;
        root.insert(&3);
        root.insert(&7);
        root.insert(hehe);
        root.insert(&4);
        root.insert(haha);
        root.insert(&8);

        assert_eq!(root.val, 5);
        assert_eq!(root.left.as_ref().unwrap().val, 3);
        assert_eq!(root.right.as_ref().unwrap().val, 7);
        assert_eq!(root.left.as_ref().unwrap().left.as_ref().unwrap().val, 2);
        assert_eq!(root.left.as_ref().unwrap().right.as_ref().unwrap().val, 4);
        assert_eq!(root.right.as_ref().unwrap().left.as_ref().unwrap().val, 6);
        assert_eq!(root.right.as_ref().unwrap().right.as_ref().unwrap().val, 8);
    }


    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    struct Test{
        val: i32,
    }

    #[test]
    fn test_insert_2() {
        let mut root: TreeNode<Test> = TreeNode::new(Test{val: 5});
        let haha = Test{val: 6};
        let hehe = Test{val: 2};
        root.insert(&Test{val: 3});
        root.insert(&Test{val: 7});
        root.insert(&hehe);
        root.insert(&Test{val: 4});
        root.insert(&haha);
        root.insert(&Test{val: 8});

        assert_eq!(root.val, Test{val: 5});
        assert_eq!(root.left.as_ref().unwrap().val, Test{val: 3});
        assert_eq!(root.right.as_ref().unwrap().val, Test{val: 7});
        assert_eq!(root.left.as_ref().unwrap().left.as_ref().unwrap().val, Test{val: 2});
        assert_eq!(root.left.as_ref().unwrap().right.as_ref().unwrap().val, Test{val: 4});
        assert_eq!(root.right.as_ref().unwrap().left.as_ref().unwrap().val, Test{val: 6});
        assert_eq!(root.right.as_ref().unwrap().right.as_ref().unwrap().val, Test{val: 8});
    }

    #[test]
    fn test_dealership(){
        let mut dealer = CarDealer::new(vec![Car::new("Ferrari".to_string()), Car::new("Lamborghini".to_string())]);
        dealer.add_car(Car::new("Porsche".to_string()));
        dealer.print_cars();
        let mut user = User::new();
        dealer.rent_user(&mut user, "Ferrari".to_string());
        user.print_car();
        dealer.end_rental(&mut user);
        user.print_car();
    }

    #[test]
    fn test_entangled_bit(){
        let mut bit1 = EntangledBit::default();
        let mut bit2 = EntangledBit::default();
        let mut bit3 = EntangledBit::default();
        bit1.set();
        assert_eq!(bit1.get(), true);
        assert_eq!(bit2.get(), false);
        assert_eq!(bit3.get(), false);
        bit1.entangle(&mut bit2);
        bit1.entangle(&mut bit3);
        assert_eq!(bit1.get(), true);
        assert_eq!(bit2.get(), true);
        assert_eq!(bit3.get(), true);
        bit1.reset();
        assert_eq!(bit1.get(), false);
        assert_eq!(bit2.get(), false);
        assert_eq!(bit3.get(), false);
    }
}