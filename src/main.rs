mod debug;
mod enums;
mod errors;
mod generics;
mod lifetime;
mod ownership;
mod stack_heap;
mod structs;
mod traits;
mod unit_test;
mod vars;

extern crate lib_demo;

const MAX_POINT: u32 = 1000000;

fn main() {
    println!("Hello, world!");
    vars::run();
    // vars::sub_a::func_a();
    // vars::sub_b::func_b();

    let mut x = 5;
    println!("The val x is {}", x);
    x = 6;
    println!("The val x is {}", x);

    let _i1 = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS);
    println!("The address of MAX_POINT is {:p}", &MAX_POINT);

    let i2: i64 = 1;
    let i3: i64 = 2;

    println!("Stack i2 is {:p}", &i2);
    println!("Stack i3 is {:p}", &i3);

    let y = 5;
    println!("Stack before y is {:p}", &y);
    let y = y + 1;
    println!("Stack after y is {:p}", &y);

    let y = y * 2;

    {
        let y = 0;
        println!("Scoped y is {}", y);
    }
    println!("Out Scoped y is {}", y);

    let t1 = (500, 0.6, "Hello");
    let (_x, _y, _z) = t1;
    println!("The values of t1 are {}, {}, {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("{:?}", t2);

    let a1 = [1, 3, 5, 7, 8];
    let a2 = [0; 10];
    println!("{:?}, {:?}, {}, {}", a1, a2, a1[2], a1[3]);

    let s1 = "helloこんにちはWorld";
    let s2 = "hello";
    println!("Stack address of s1 is {:p}", &s1);
    println!("Stack address of s2 is {:p}", &s2);
    println!("Static address of s1 is {:?}", s1.as_ptr());
    println!("Static address of s2 is {:?}", s2.as_ptr());
    println!("Length of s1 is {:?}", s1.len());
    println!("Length of s2 is {:?}", s2.len());

    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("Stack address of s1 is {:p}", &s1);
    println!("Stack address of s2 is {:p}", &s2);
    println!("Static address of s1 is {:?}", s1.as_ptr());
    println!("Static address of s2 is {:?}", s2.as_ptr());
    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{}, {}", s1, s2);

    stack_heap::run();

    ownership::run();

    generics::run();

    lifetime::run();

    structs::run();

    enums::run();

    traits::run();

    errors::run();

    lib_demo::print_random_number();

    debug::run();
}
