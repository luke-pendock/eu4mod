use crate::expression::*;
use crate::identifier::*;

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
        }
    }
}