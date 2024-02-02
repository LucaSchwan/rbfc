extern crate rbfc;

use rbfc::parser;

fn main() {
    let input = String::from("+++[->+<]...,,,");
    let mut parser = parser::Parser::new(input);
    let ops = parser.parse();
    for op in ops {
        println!("{:?}", op);
    }
}
