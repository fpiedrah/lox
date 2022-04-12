mod error;
mod expression;
mod scanner;
mod token;

use crate::scanner::Scanner;

fn main() {
    let mut scanner = Scanner::new("\"!=>=()//()\" 10 and some=10".to_string());

    println!("{:?}", scanner.scan_tokens());
}
