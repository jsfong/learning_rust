#![allow(dead_code)]
#![allow(unused_variables)]
mod collections_type;
mod control_flow;
mod data_struct;
mod generics;
mod pm;
mod sh;

use std::mem;

//Global variable
const MEANING_OF_LIFE: u8 = 42; //no fixed address - prefer

static STATIC_Z: i32 = 123; //Immutable is memory safe

static mut STATIC_UNSAFE_Z: i32 = 123; //Mutable is memory unsafe

fn main() {
    println!("Hello, world!");
    // core_data_type();
    //
    // operators();
    //
    // scope_and_shadowing();
    //
    // constants();

    // sh::stack_and_heap();

    // control_flow::for_loop();

    // control_flow::match_statement();

    // data_struct::structures();

    // data_struct::enums();

    // data_struct::union_test();

    // data_struct::option_test();

    // data_struct::array();

    // data_struct::slices();

    // data_struct::tuples();

    // pm::pattern_matching();

    // generics::generics();

    collections_type::vectors();
}

fn core_data_type() {
    //let --> const & immutable
    // u = unsigned, 8 bits, 0-255 (0 to 2^N-1)
    let a: u8 = 123;
    println!("a = {}", a); // immutable

    //let mut --> Mutable
    // i = signed, 8 bits, -128 -- 127 (-2^N-1 to 2^(N-1) - 1)
    let mut b: i8 = 0;
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    //Type inference
    let c = 123456789; //i32 = 32 bits = 4 bytes
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    // u8, u16, u32, u64, i8, i16, ...

    // usize isize
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    // Character
    let d: char = 'x';
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    // f32 f64 IEEE-754 all signed
    let e: f32 = 2.5;
    println!("{}, size = {} bytes", e, mem::size_of_val(&e));

    // Boolean
    let g: bool = false;
    println!("{}, size = {} bytes", g, mem::size_of_val(&g));
}

fn operators() {
    //arithmetic
    let mut a = 2 + 3 * 4; //+-*
    println!("{}", a);
    a = a + 1; // not support -- ++
    a -= 2; // a = a -2; -= += *= /= %=
    println!("remainder of {} / {} is  {}", a, 3, (a % 3));

    // Power
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise *only for integer
    let c = 1 | 2; // | OR   & AND  ^ XOR  ! NOR
                   // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);

    // Shift
    let two_to_10 = 1 << 10; // >>
    println!("2^10 = {}", two_to_10);

    // Logical > <= >= ==
    let _pi_less_4 = std::f64::consts::PI < 4.0; //true
    let x = 5;
    let _x_is_5 = x == 5; // true
}

fn scope_and_shadowing() {
    let a = 123;
    println!("a = {}", a);

    //Create a scope
    {
        let b = 456;
        println!("inside b = {}", b);

        //shadowing
        let a = 777;
        println!("inside a = {}", a);
    }
    println!("outside a = {}", a);
}

fn constants() {
    unsafe {
        println!("{}", STATIC_UNSAFE_Z);
    }
}
