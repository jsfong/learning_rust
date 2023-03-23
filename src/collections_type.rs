#![allow(dead_code)]

pub fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);

    //Type of the index must be usize (size of the machine)
    let idx: usize = 0;
    println!("a[0] = {}", a[idx]);

    a[idx] = 312;
    println!("a[0] = {}", a[idx]);

    //Option
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element"),
    }

    //Iterating
    for x in &a {
        println!("{}", x);
    }

    //Add
    a.push(44);
    println!("{:?}", a);

    //Remove
    let last_elem = a.pop(); //option
    println!("laste elemt is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}
