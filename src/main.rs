extern crate rbfc;

use rbfc::parser;

fn main() {
    let input = String::from("+++[->+<]...,,,");
    let mut parser = parser::Parser::new(input);
    let mut ops = parser.parse().unwrap();
    while let Some(op) = ops.pop() {
        println!("{:?}", op);
    }
}
