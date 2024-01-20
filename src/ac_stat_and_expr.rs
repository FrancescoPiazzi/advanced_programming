#![allow(dead_code)]

fn main() {
    let _a = 5; // statement

    let y = {
        let x = 3; // statement
        x + 1 // expression
    }; // statement

    println!("y: {}\n", y); // y = 4

    // expressions can also be used inside if else blocks by themeselvs
    let _b = -2;
    let _bsign = if _b > 0 {
        1
    } else if _b == 0 {
        0
    } else {
        -1
    };

    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("");
    for element in (1..5).rev() {
        println!("{}", element);
    }
}
