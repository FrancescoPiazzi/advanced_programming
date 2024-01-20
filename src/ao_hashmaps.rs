#![allow(dead_code)]


use std::collections::HashMap;

fn main() {
    let keys = ["a", "b", "c", "d", "e", "f", "g", "h"];
    let values = [1, 2, 3, 4, 5, 6, 7, 8];

    // map from key value pairs
    let map1: HashMap<&str, i32> = HashMap::from_iter(keys
        .iter()
        .cloned()
        .zip(
            values.iter().cloned()
        ));

    // map from key value pairs, skipping the first half of the keys
    let map2: HashMap<&str, i32> = HashMap::from_iter(keys
        .iter()
        .cloned()
        .skip(keys.len()/2)
        .zip(
            values.iter().cloned().skip(keys.len()/2)
        ));

    // map from key value pairs, skipping the first half of the keys and second half of the values
    let map3: HashMap<&str, i32> = HashMap::from_iter(keys
        .iter()
        .cloned()
        .skip(keys.len()/2)
        .zip(
            values.iter().cloned()
        ));

    println!("map1: {:?}", map1);
    println!("map2: {:?}", map2);
    println!("map3: {:?}", map3);

    // copy of map 1, but make every value even by adding 1 to the odd values
    let mapeven1: HashMap<&str, i32> = map1
        .clone()
        .into_iter()
        .map(|(k, v)| (k, if v%2!=0 {v+1} else {v}))
        .collect();

    // copy of map 1, but filter out the odd values
    let mapeven2: HashMap<&str, i32> = map1
        .clone()
        .into_iter()
        .filter(|(_, v)| *v%2 == 0)
        .collect();

    println!("mapeven1: {:?}", mapeven1);
    println!("mapeven2: {:?}", mapeven2);

}
