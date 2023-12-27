mod print_expression;
mod expression;
mod operator;
mod statement;
mod parser;
mod identifier;

use std::collections::HashMap;
use chumsky::prelude::*;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let empty_map: HashMap<&identifier::Identifier, f64> = HashMap::new();

    let src_ast = parser::parser().parse(src.clone());
    println!("src_ast: {:?}", src_ast);

    match parser::parser().parse(src) {
        Ok(ast) => match parser::eval_statement(&ast,  &empty_map) {
            Ok(output) => println!("{}", output),
            Err(eval_err) => println!("Evaluation error: {}", eval_err),
        },
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|err| println!("Parse error: {}", err)),
    }
    //println!("{:?}", parser::parser().parse(src));
}
