pub fn run() {
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{}", s2);

    let i1 = 1;
    let i2 = i1;

    println!("{}, {}", i1, i2);
    println!("Stack memory address of i1 is {:p}", &i1);
    println!("Stack memory address of i2 is {:p}", &i2);

    let sl1 = "aiue";
    let sl2 = sl1;
    println!("Stack memory address of sl1 is {:p}", &sl1);
    println!("Stack memory address of al2 is {:p}", &sl2);

    let s3 = String::from("Hello");
    let s4 = s3.clone();
    println!("Stack memory address of s3 is {:p}", &s3);
    println!("Stack memory address of s4 is {:p}", &s4);
    println!("Heap memory address of s3 hello is : {:?}", s3.as_ptr());
    println!("Heap memory address of s4 hello is : {:?}", s4.as_ptr());

    let s5 = String::from("Hello");
    println!("Stack memory address of s5 is {:p}", &s5);
    println!("Heap memory address of s5 is {:?}", s5.as_ptr());
    println!("Len of s5 is {}", s5.len());
    println!("Cap of s5 is {}", s5.capacity());
    take_ownership(s5);
    // println!("{}", s5);

    let s6 = String::from("Hello0");
    println!("Stack memory address of s6 is {:p}", &s6);
    println!("Heap memory address of s6 is {:?}", s6.as_ptr());
    println!("Len of s6 is {}", s6.len());
    println!("Cap of s6 is {}", s6.capacity());
    let s7 = tack_giveback_ownership(s6);
    println!("Stack memory address of s7 is {:p}", &s7);
    println!("Heap memory address of s7 is {:?}", s7.as_ptr());
    println!("Len of s7 is {}", s7.len());
    println!("Cap of s7 is {}", s7.capacity());

    let s8 = String::from("Hello");
    let len = calculate_length(&s8);
    println!("Length of {} is {}", s8, len);

    let mut s9 = String::from("Hello");
    change(&mut s9);
    println!("changed s9 is {}", s9);

    let s10 = String::from("Hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{}, {}, {}", s10, r1, r2);

    // NG
    // let mut s10 = String::from("Hello");
    // let r1 = &s10;
    // let &mut r2 = &s10;
    // println!("{}, {}, {}", s10, r1, r2);

    let mut s11 = String::from("Hello");
    let r1 = &mut s11;
    println!("{}", r1);
    println!("{}", s11);

    let mut s12 = String::from("Hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{}", r1);
    println!("{}", r2);
    let r3 = &mut s12;
    *r3 = String::from("updated");
    println!("{}", s12);

    // let r;
    // {
    //     let x = 1;
    //     r = &x
    // }
    // println!("{}", r);
}

fn take_ownership(s: String) {
    println!("take_ownership");
    println!("{}", s);
    println!("Stack memory address of s is {:p}", &s);
    println!("Heap memory address of s is {:?}", s.as_ptr());
    println!("Len of s is {}", s.len());
    println!("Cap of s is {}", s.capacity());
}

fn tack_giveback_ownership(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_world");
}
