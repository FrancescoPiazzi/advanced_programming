#![allow(dead_code)]

fn prev_str(s1: &str) -> String {
    let s2: String = s1.chars().map(|x| match x { 
        'b'..='z' | 'B'..='Z' => char::from_u32(x as u32 -1).unwrap(),
        _ => x
    }).collect();
    s2
}



struct X{
    s: Option<String>,
    i: i32
}

impl X{
    fn new(s1: &str, i1: i32) -> X{
        let x = X{
            s:Some(s1.to_string()),
            i:i1
        };
        x
    }

    fn take_str(&mut self) -> Option<String>{
        let res = self.s.clone();
        self.s = None;
        res
    }
}



struct NameSurname{
    name: String,
    surname: String
}

fn replace_surname(mut name: NameSurname, s: String) -> String{
    let mut res = s.clone();
    std::mem::swap(&mut name.surname, &mut res);
    res
}

#[derive(Clone)]
struct Student{
    name: String,
    id: u32
}

impl Student{
    
    fn new(name: String, id: u32) -> Student{
        Student { name: name, id: id }
    }
}

impl std::fmt::Display for Student {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "name: {}, id: {}", self.name, self.id)
    }
}


struct University{
    name: String,
    students: Vec<Student>
}


impl University {
    fn new(s: &str, student: &[Student]) -> University{
        University { name: s.to_string(), students: Vec::from(student) }
    }

    fn remove_student(&mut self, id: u32) -> Result<Student, &str>{
        if self.students.len() <= id as usize{
            return Err("Not found");
        }
        else{
            return Ok(self.students.remove(id as usize));
        }
    }
}


fn main() {
    println!("{}", prev_str("Aloha, world!"));

    let mut x = X::new("ciao", 10);
    let s = x.take_str();
    println!("{}", s.unwrap());

    let person1: NameSurname = NameSurname {
        name: "Ernesto".to_string(),
        surname: "Bianchi".to_string(),
    };
    let surname = "sassi".to_string();
    let a = replace_surname(person1, surname);
    println!("{}", a);


    /*
    let s1 = Student::new("marco", 1);
    let s2 = Student::new("anto", 2);
    let s3 = Student::new("anna", 3);
    let mut university = University::new("Trento", &vec![s1, s2, s3]);

    println!("{}", university);

    println!("{}", university.remove_student(1).unwrap().id);
    */

    /*
    let mut fleet = AirFleet{
        fleet: Vec::new(),
    };

    let airplane1 = Airplane{
        company: AirplaneCompany::Airbus,
        model: "A380".to_string(),
    };

    let airplane2 = Airplane{
        company: AirplaneCompany::Boeing,
        model: "747".to_string(),
    };

    let airplane3 = Airplane{
        company: AirplaneCompany::Airbus,
        model: "A320".to_string(),
    };

    fleet.add_airplane(airplane1);
    fleet.add_airplane(airplane2);
    fleet.add_airplane(airplane3);

    println!("{:?}", fleet.search_airplane("A380"));
    println!("{:?}", fleet.search_airplane("747"));
    println!("{:?}", fleet.search_airplane("A320"));
    println!("{:?}", fleet.search_airplane("A330"));
    */

    /*
    let mut hashmap = HashMap::new();
    hashmap.insert(1, "ciao".to_string());
    hashmap.insert(2, "ciao".to_string());
    hashmap.insert(3, "ciao".to_string());

    let hashmap = Maps{
        map: hashmap,
    };
    let hashmap = string_to_tuple(hashmap);
    println!("{:?}", (hashmap.get(&1).unwrap().0, hashmap.get(&1).unwrap().1.clone()));
    */

    /*
    use std::fmt::{Debug, Formatter};

    impl Debug for Size {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Size {{ width: {}, height: {} }}", self.width, self.height)
        }
    }

    let s = Size::new(5.7, 1.2);

    println!("{:?}", s.area());
    println!("{:?}", s.compare(&Size::new(8.9, 10.)));
    println!("{:?}", s.compare(&Size::new(1.8, 0.1)));
    println!("{:?}", s.compare(&Size::new(5.7, 1.2)));
    */

    /*
    let x = MaybePoint::new(Some(10),Some(20));
    let y = MaybePoint{x:Some(10),y:None};

    println!("{:?}", x.is_some());
    println!("{:?}", y.is_some());
    println!("{:?}", x.maybe_len());
    println!("{:?}", y.maybe_len());
    */

    /*
    println!("{:?}", wrapper(10));
    println!("{:?}", wrapper(5));
    println!("{:?}", wrapper(11));
    */

    /*
    let a :Vec<String> = vec!["Ciao".to_string(), "Come".to_string(), "Va".to_string()];

    let b = order(a);
    println!("{}", b.get(1).unwrap().to_owned());
    println!("{}", b.get(2).unwrap().to_owned());
    */

    /*
    use crate::modsum::sum;
    pub fn main() {
    println!("{}", sum(modx::X::S(' '), mody::X::F(1.2, 4)));
    println!("{}", sum(modx::X::C("hello".to_owned()), mody::X::F(2.4, 10)));
    }
    */
}