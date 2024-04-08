struct Point<T> {
    x: T,
    y: T,
}

struct PointAnother<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    let number_list = vec![34, 5, -111, 4873, 17];

    // println!("largest number is {}", largetst_i32(number_list));

    let char_list = vec!['a', 'b', 'c', 'd'];
    println!("largest number is {}", largetst_generics(number_list));
    println!("largest char is {}", largetst_generics(char_list));

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3.0, y: 4.0 };
    let p3 = PointAnother { x: 1, y: 10.4 };
    let p4 = PointAnother {
        x: "Rust",
        y: "aaa",
    };
    let p5 = p3.mixup(p4);
    println!("{}, {}", p5.x, p5.y);
}

fn largetst_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largetst_generics<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
