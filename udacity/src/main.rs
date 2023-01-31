fn main() {
    println!("{}",naive(2,3));
}

fn naive(a: i32, b: i32) -> i32 {
    let mut x = a;
    let y = b;
    let mut z = 0;
    while x > 0 {
        z = z + y;
        x = x - 1;
    }
    z
}
