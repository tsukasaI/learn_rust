enum List {
    Node(i32, Box<List>),
    Nil,
}

pub fn run() {
    println!("--------here is stack_heap mod--------------");

    // the below code causes stack over flow
    // let a1: [u8; 9000000] = [1; 9000000];

    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("Stack address of v1 is {:p}", &v1);
    println!("Stack address of v2 is {:p}", &v2);
    println!("Heap memory address of v1 is {:?}", v1.as_ptr());
    println!("Len of v1 is {}", v1.len());
    println!("Cap of v1 is {}", v1.capacity());
    v1.insert(1, 10);
    println!("{:?}", v1);
    v1.remove(0);
    println!("{:?}", v1);
    v1.append(&mut v3);
    println!("v1 {:?}", v1);
    println!("v3 {:?}", v3);

    let t1: (i64, String) = (10, String::from("hello"));
    println!("Stack address of t1 is {:p}", &t1);
    println!("Heap memory address of t1.1 is {:?}", t1.1.as_ptr());
    println!("Len of t1.1 is {}", t1.1.len());
    println!("Cap of t1.1 is {}", t1.1.capacity());

    let mut b1 = Box::new(t1);
    (*b1).1 += " world";
    println!("b1 is {:?}", b1);
    println!("Stack address of b1 is {:p}", &b1);
    println!("Heap memory address of b1.1 is {:p}", b1);
    println!("Len of b1.1 is {}", b1.1.len());
    println!("Cap of b1.1 is {}", b1.1.capacity());
}
