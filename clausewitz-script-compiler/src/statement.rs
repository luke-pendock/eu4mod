use crate::expression::*;
use crate::identifier::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Argument {
    name: String,
    argument_type: Type,
    //optional: bool,
}

impl Argument {
    pub fn as_string(&self) -> String {
        format!("{}: {}", self.name, self.argument_type.as_string())
    }
}

#[derive(PartialEq, Debug, Clone)]
struct ArgumentList { 
    arguments: Vec<Argument>
}

impl ArgumentList {
    pub fn as_string(&self) -> String {
        self.arguments
            .iter()
            .map(|arg| format!("{}: {}", arg.name, arg.argument_type.as_string()))
            .collect::<Vec<String>>()
            .join(", ")
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let {
        identifier: Identifier, 
        expression: Expression,
        then: Box<Statement>,
    },
    IfElse {
        condition: Box<Expression>, 
        body: Box<Statement>,
        else_body: Box<Statement>,
        then: Box<Statement>,
    },
    FunctionDefinition {
        name: String,
        args: ArgumentList,
        return_type: Type,
        body: Box<Statement>,
        then: Box<Statement>,
    },
    Expression(Expression),
}

impl Statement {
    pub fn as_string(&self) -> String {
        use Statement::*;
        match self {
            Let {
                identifier, 
                expression,
                then
            } => format!(
                "let {} = {}\n{}", 
                identifier.as_string(), 
                expression.as_string(), 
                then.as_string()
            ),
            IfElse {
                condition, 
                body, 
                else_body,
                then
            } => format!(
                "if {} {{ {} }} else {{ {} }}\n{}", 
                condition.as_string(), 
                body.as_string(), 
                else_body.as_string(), 
                then.as_string()
            ),
            Expression(expression) => expression.as_string(),
            FunctionDefinition {
                name,
                args,
                body,
                then,
            } => format!(
                "fn {} ({}) {{ \n }} {}", name, args.as_string(), then.as_string(),
            ),
            _ => todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn adder_function () -> Statement { 
        Statement::FunctionDefinition {
            name: "adder".to_string(),
            args: ArgumentList {
                arguments: vec![
                    Argument {
                        name: "a".to_string(),
                        value_type: Type::Int,
                    },
                    Argument {
                        name: "b".to_string(),
                        value_type: Type::Int,
                    }
                ]
            },
            return_type: Type::Int,
            body: Box::new(Statement::Expression(Expression::BinaryExpression {
                left: Box::new(Expression::Identifier(Identifier::Variable("a".to_string()))),
                right: Box::new(Expression::Identifier(Identifier::Variable("b".to_string()))),
            })),
        }
    }
}