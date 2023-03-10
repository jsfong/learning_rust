#![allow(dead_code)]

use std::mem;

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

pub fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("Point p is at ({}, {})", p.x, p.y);

    let p2: Point = Point { x: 3.0, y: 4.0 };
    let my_line = Line { start: p, end: p2 };
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), //tuple
    Cmyk {
        //struct
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

pub fn enums() {
    let c: Color = Color::Cmyk {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0)
        | Color::Cmyk {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb ({}, {}, {})", r, g, b),
        _ => (),
    }
}

//Specify a block of 32 bits memory
//Which can either hold i32 or f32
union IntOrFloat {
    i: i32,
    f: f32,
}

pub fn union_test() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    // process_value(IntOrFloat { i: 42 });
    process_value(IntOrFloat { f: 42.0 });
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value");
            }

            IntOrFloat { f } => {
                println!("value = {}", f);
            }
        }
    }
}

pub fn option_test() {
    let x = 3.0;
    let y = 0.0;

    //Option -> Some(v) | None
    let result = if y != 0.0 { Some(x / y) } else { None };

    //Check
    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Cannot divide by zero"),
    }

    if let Some(z) = result {
        println!("result = {}", z);
    }

    // while let
}

pub fn array() {
    //fix size
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    a[0] = 321;

    println!("a has {} elements, first is {}", a.len(), a[0]);

    //print the whole array
    println!("{:?}", a);

    if a != [1, 2, 3, 4, 5] {
        print!("does not match");
    }

    let b = [1u16; 10];
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

pub fn slices() {
    //actual size is not know at compile time

    let mut data = [1, 2, 3, 4, 5];

    use_slice(&mut data[1..4]);
    // use_slice(&mut data);
    println!("{:?}", data);
}

fn use_slice(slice: &mut [i32]) {
    println!("first element = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

pub fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);

    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c,d), (e,f)) = combined;
    
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    let meaning = (42,);
    println!("{:?}", meaning);


}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}
