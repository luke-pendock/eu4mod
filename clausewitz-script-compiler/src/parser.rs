use std::collections::HashMap;

use crate::expression::*;
use chumsky::{
    prelude::*, 
};
use crate::operator::*;
use crate::statement::*;
use crate::identifier::*;

pub fn parser() -> impl Parser<char, Statement, Error = Simple<char>> {

    let ident = text::ident()
        .padded();

    let expr = expr_parser();

    let statement_expression = expr
        .clone()
        .map(|expr| Statement::Expression(expr));
    
    let decl: Recursive<'_, char, Statement, Simple<char>> = recursive(|decl| {
        let r#let = 
            text::keyword("let")
            .ignore_then(ident)
            .then_ignore(just('='))
            .then(expr.clone())
            .then_ignore(just(';'))
            .then(decl)
            .map(|((var, rhs), next)| Statement::Let {
                identifier: Identifier {
                    name: var,
                },
                expression: rhs,
                then: Box::new(next)
            });
        r#let
            .or(statement_expression)
            .padded()
    });

    decl.then_ignore(end())
}

fn expr_parser() -> Recursive<'static, char, Expression, Simple<char>> {
    // https://doc.rust-lang.org/reference/expressions.html
    // Order of operations:
    // Paths
    // Method calls
    // Function calls, array indexing
    // ?
    // Unary
    // as
    // * / %
    // + -
    // << >>
    // &
    // ^
    // |
    // == != < <= > >=
    // &&
    // ||
    // .. ..=
    // += -= *= /= %= &= |= ^= <<= >>= =
    recursive(|expr| {
        let ident = text::ident()
            .padded();

        let int = text::int(10)
            .map(|s: String| Expression::Num(s.parse().unwrap()))
            .padded();

        let atom = int
            .or(expr.delimited_by(just('('), just(')')))
            .or(ident.map(|x| Expression::Variable(Identifier{ name: x })))
            .padded();

        let op = |c| just(c).padded();

        let unary = op('-').repeated().then(atom)
            .foldr(|_op, rhs| Expression::UnaryExpression { 
                operator: UnaryOperator::Negative, 
                expression: Box::new(rhs) 
            });

        let bin_fn = |op| |a, b| Expression::BinaryExpression { 
            operator: op, 
            left_expression: a, 
            right_expression: b 
        };

        let product = unary.clone()
            .then(op('*').to(bin_fn(BinaryOperator::Mul))
                .or(op('/').to(bin_fn(BinaryOperator::Div))
                .or(op('%').to(bin_fn(BinaryOperator::Mod))))
                .then(unary)
                .repeated())
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum =product.clone()
            .then(op('+').to(bin_fn(BinaryOperator::Add))
                .or(op('-').to(bin_fn(BinaryOperator::Sub)))
                .then(product)
                .repeated())
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));
    
        sum
    })
}

// fn int_parser() -> impl Parser<char, Expression, Error = Simple<char>> {
//     let int =text::int(10)
//         .map(|s: String| Expression::Num(s.parse().unwrap()))
//         .padded();  
//     return int.then_ignore(end());
// }

// fn atom_parser() -> impl Parser<char, Expression, Error = Simple<char>>  {
//     return int_parser();
// }

// fn operator_parser() -> impl Fn(char) -> Padded<Just<char, char, Simple<char>>> {
//     return |c| just(c).padded();
// }

// fn unary_parser() -> impl Parser<char, Expression, Error = Simple<char>> {
//     use crate::operator::UnaryOperator::*;
//     use crate::expression::*;
//     let op = operator_parser(); 
//     let atom = atom_parser()
//     let unary = op('-').repeated().then(atom)
//         .foldr(|_op, rhs| Expression::UnaryExpression { 
//             operator: Negative, 
//             expression: Box::new(rhs) 
//         });
//     return unary.then_ignore(end());
// }

pub fn eval_expression<'a> (expr: &'a Expression, vars: &HashMap<&'a Identifier, f64>) -> Result<f64, String> {
    match expr {
        Expression::Num(x) => Ok(*x),
        Expression::UnaryExpression { operator, expression } => {
            use crate::operator::UnaryOperator::*;
            let x = eval_expression(expression, vars)?;
            match operator {
                Negative => Ok(-x),
                _ => todo!(),
            }
        },
        Expression::BinaryExpression { operator, left_expression, right_expression } => {
            use crate::operator::BinaryOperator::*;
            let x = eval_expression(left_expression, vars)?;
            let y = eval_expression(right_expression, vars)?;
            match operator {
                Add => Ok(x + y),
                Mul => Ok(x * y),
                Div => Ok(x / y),
                Mod => Ok(x % y),
                Sub => Ok(x - y),
                _ => todo!(),
            }
        },
        Expression::Variable(x) => {
            if vars.contains_key(x) {
                Ok(vars[x])
            } 
            else {
                Err(format!("Variable not found: {}", x.name))
            }
        },
        _ => todo!(),
    }
}

pub fn eval_statement<'a> (statement: &'a Statement, vars: &HashMap<&'a Identifier, f64>) -> Result<f64, String> {
    use crate::statement::Statement::*;
    match statement {
        Let { identifier, expression, then } => {
            let rhs = eval_expression(expression, vars);
            let mut new_vars = vars.clone();
            new_vars.insert(identifier, rhs?);
            
            return eval_statement(then, &new_vars);
        },
        Expression(expr) => {
            return eval_expression(expr, vars);
        }
        _ => todo!(),
    }
}