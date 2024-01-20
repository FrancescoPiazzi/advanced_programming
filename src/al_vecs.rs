#![allow(dead_code)]

fn main() {
    // eliminazione di un elemento da un vettore
    let mut v: Vec<String> = Vec::from(["haha".to_string(), "hehe".to_string(), "waaa".to_string(), "haha".to_string(), "banana".to_string()]);
    let to_remove = "hehe".to_string();
    println!("{:?}", v);

    // modo compatto che fa casino se l'elemento non c'è
    v.remove(v.iter().position(|x| *x == to_remove).unwrap());
    println!("{:?}", v);

    // modo lungo che gestisce anche il caso in cui l'elemento non c'è
    if let Some(pos) = v.iter().position(|x| *x == to_remove) {
        v.remove(pos);
    } else {
        println!("not found");
    }
    println!("{:?}", v);
    
    let to_remove = "haha".to_string();
    // rimuove tutte le occorrenze, non fa nulla se non trova nessuna occorrenza
    v.retain(|x| *x != to_remove);
    println!("{:?}", v);

}
