pub fn run() {
    println!("Lifetime");

    let st1 = String::from("x");
    let st2 = String::from("y");
    let res1 = get_longest(&st1, &st2);
    println!("res1: {}", res1);

    let st3 = String::from("x");
    let res2;
    {
        let st4 = String::from("y");
        get_longest(&st3, &st4);
        res2 = get_longest(&st1, &st2);
        println!("res1: {}", res2);
    }
}

fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// NG
// fn dummy<'a>() -> &'a str {
//     let s = String::from("demo");
//     &s
// }
