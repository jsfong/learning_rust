#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;

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

pub fn hashmap() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);
    println!("a square has {} sides", shapes["square".into()]);

    for (key, value) in &shapes {
        println!("{} : {}", key, value);
    }

    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);

    //Insert if not exist
    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}", shapes);

    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes);
}

pub fn set() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    greeks.insert("delta"); //nothing will happen
    println!("{:?}", greeks);

    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("We added vega! Hoory!")
    }

    if !greeks.contains("kappa") {
        println!("We don't have kappa");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("We removed delta");
    }
    println!("{:?}", greeks);

    //Set
    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    //Subset
    println!(
        "is {:?} a subset of {:?}? {}",
        _2_8,
        _1_10,
        _2_8.is_subset(&_1_10)
    );

    //disjoint = no common elements
    println!(
        "is {:?} a disjoint of {:?}? {}",
        _1_5,
        _6_10,
        _1_5.is_disjoint(&_6_10)
    );

    //union, intersection
    println!(
        "is {:?} a union of {:?}? {:?}",
        _2_8,
        _6_10,
        _2_8.union(&_6_10)
    );

    //difference
}

pub fn iterator() {
    let mut vec = vec![3, 2, 1];

    for x in &vec {
        //use ref to borrow without changing the vec
        println!("{}", x);
    }

    for x in vec.iter() {
        //to let x to be mut use vec.iter_mut()
        println!("We got {}", x); //x is immutable
    }

    for x in vec.iter_mut() {
        *x += 2;
    }
    println!("{:?}", vec);

    for x in vec.iter().rev() {
        println!("in reverse: {}", x);
    }

    let mut vec2 = vec![1, 2, 3];

    // let it = vec.into_iter();
    vec2.extend(vec);
    println!("{:?}", vec2);
}
