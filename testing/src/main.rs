fn main() {
    let f: fn(i32) -> i32 = add_one;
    let six = f(5);
    println!("six: {}", six);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverges() -> ! {
    panic!("This function never returns!");
}