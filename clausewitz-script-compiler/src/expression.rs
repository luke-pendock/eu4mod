use std::vec;
use crate::{operator::*, identifier::Identifier};

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Boolean,
    Int,
    Float,
    Char,
    String,
    DataStructure
}

impl Type {
    pub fn as_string(&self) -> String {
        match self {
            Type::Boolean => String::from("boolean"),
            Type::Int => String::from("int"),
            Type::Float => String::from("float"),
            Type::Char => String::from("char"),
            Type::String => String::from("string"),
            Type::DataStructure => String::from("data structure")
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Value {
        value_type: Type,
        value: String
    },
    Num(f64),

    Variable(Identifier),

    UnaryExpression {
        operator: UnaryOperator,
        expression: Box<Expression>
    },
    BinaryExpression { 
        operator: BinaryOperator,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>
    },
}

impl Expression {
    pub fn as_string(&self) -> String {
        use Expression::*;
        let nested_expression = | expression: &Expression | -> String {
            match expression {
                Value => expression.as_string(),
                _ => format!("({})", expression.as_string())
            }
        };
        match self {
            Value { value_type, value } => {
                match value_type {
                    Type::String => format!("\"{}\"", value),
                    _ => value.clone()
                }
            }
            UnaryExpression { operator, expression } => {
                format!("{}{}", operator.as_string(), nested_expression(expression))
            }
            BinaryExpression { operator, left_expression, right_expression } => {
                format!("{}{}{}", nested_expression(left_expression), operator.as_string(), nested_expression(right_expression))
            }
            Num(num) => num.to_string(),
            Variable(var) => var.as_string()
        }
    }
}

// TODO: Implement tests for And Operator
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_string_value_string() {
        let expression = Expression::Value {
            value_type: Type::String,
            value: String::from("hello"),
        };
        assert_eq!(expression.as_string(), String::from("\"hello\""));
    }

    #[test]
    fn test_as_string_value_other_type() {
        let expression = Expression::Value {
            value_type: Type::Int,
            value: String::from("42"),
        };
        assert_eq!(expression.as_string(), String::from("42"));
    }

    #[test]
    fn test_as_string_unary_expression() {
        let expression = Expression::UnaryExpression {
            operator: UnaryOperator::Negative,
            expression: Box::new(Expression::Value {
                value_type: Type::Int,
                value: String::from("42"),
            }),
        };
        assert_eq!(expression.as_string(), String::from("-42"));
    }

    #[test]
    fn test_as_string_binary_expression() {
        let expression = Expression::BinaryExpression {
            operator: BinaryOperator::Add,
            left_expression: Box::new(Expression::Value {
                value_type: Type::Int,
                value: String::from("1"),
            }),
            right_expression: Box::new(Expression::Value {
                value_type: Type::Int,
                value: String::from("2"),
            }),
        };
        assert_eq!(expression.as_string(), String::from("1+2"));
    }
}