use std::io;
use std::io::Write;
use crate::calcmath::calc::{Calculator, Console};

mod parsemath;
mod calcmath;

struct Terminal {
}

impl Console for Terminal {
    fn readline(&self) -> String {
        io::stdout().flush().unwrap(); // Ensure the prompt is printed

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input.trim().into()
    }

    fn println(&self, text: &str) {
        println!("{}", text);
    }

    fn print(&self, text: &str) {
        print!("{}", text);
    }
}

fn main() {

    let mut term = Terminal{};

    let mut calculator = Calculator::new(&mut term);
    calculator.run();
}
