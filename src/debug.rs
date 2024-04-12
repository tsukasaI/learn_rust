pub fn run() {
    let mut x = 1;
    x = 2;
    x = 3;

    {
        let mut y = 1;
        y = 2;
        y = 4;
    }
    x = 5;
    x = 9;
}
