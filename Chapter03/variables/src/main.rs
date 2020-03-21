const MAX_POINTS: u32 = 1000_000;

fn main() {
    fun1();
    fun2();
    fun3();
}

fn fun1() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn fun2() {
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}

fn fun3() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}

fn fun4() {
    let spaces = "   ";
    let spaces = spaces.len();
}