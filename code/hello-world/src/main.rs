static GREETING: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn increment_counter() {
    unsafe {
        COUNTER += 1;
    }
}

fn print_counter() {
    unsafe {
        println!("Counter: {}", COUNTER);
    }
}

fn main() {
    let mut message = GREETING.to_string();
    message += "!!";
    println!("{message}");

    increment_counter();
    print_counter();

    let big_number: i32 = std::i32::MAX;

    let flag = false;

    let dx;
    if flag {
        dx = 1;
    } else {
        dx = 0;
    }

    let next_number = big_number + dx;
    println!("Next number: {}", next_number);
}
