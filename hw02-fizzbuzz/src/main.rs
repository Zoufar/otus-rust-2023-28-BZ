use std::vec::Vec;

fn main() {
    let count = 100;
    let mut result = Vec::with_capacity(count);
    for i in 1..count {
        let s = match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            (_, _) => format!("{}", i),
        };
        result.push(s);
    }
    print_result(&result);
}

fn print_result(result: &Vec<String>) {
    for r in result {
        println!("{}", r)
    }
}
