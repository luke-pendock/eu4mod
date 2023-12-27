use crate::operator::*;
use crate::expression::*;

impl Expression {
    pub fn as_clausewitz(&self) -> String {
        use Expression::*;
        match self {
            Value{value_type, value} => {
                value.as_string()
            },
            BinaryExpression{operator, left, right} => {
                use Operator::BinaryOperator;
                match operator {
                    Equal => {
                        format!("")
                    },
                    _ => todo!(),
                }
                format!("{}{}{}", left.as_clausewitz(), operator, right.as_clausewitz())
            },
            UnaryExpression{operator, right} => format!("{}{}", operator, right.as_clausewitz()), 
        }
    }
}

/*
set_variable = {            
    which = <var>
    value = <number>
    which = <var> / <scope>     # Used in place of value, sets the first <var> to the second <var>'s value / <scope>'s value of <var>
}

change_variable = {            
    which = <var>
    value = <number>
    which = <var> / <scope>     # Used in place of value, adds the second <var> to the first <var> / <scope>'s value of <var> to the current scope's <var>
}

subtract_variable = {            
    which = <var>
    value = <number>
    which = <var> / <scope>     # Used in place of value, subtracts the second <var> from the first <var> / <scope>'s value of <var> from the current scope's <var>
}

*/