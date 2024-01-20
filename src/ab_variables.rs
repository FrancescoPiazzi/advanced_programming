#![allow(dead_code)]

fn main() {
    // this is ok because _a is mutable
    let mut _a = 2;
    _a = 3;

    // this is not ok, because we are changing the type of _b
    let _b  = "haha";
    // _b = 1;

    // this is ok, because we are redeclaring _c
    // which shadows the original one
    let _c = "haha";
    let _c = 1;

    // tuples are collections of elements of different types
    let _tuple = (2, "weee", 3.1415629);

    // they can be deconstructed as so
    let (_x, _y, _z) = _tuple;

    // arrays
    let _array1 = [2, 1, 4, 7];
    let _array2 = [2; 5];   // equivalent to [2, 2, 2, 2, 2]
}
