// use crate::expression::*;

// trait PrettyPrint {
//     fn pretty_print(&self) -> String;
// }

// impl PrettyPrint for Type {
//     fn pretty_print(&self) -> String {
//         "".to_string()
//     }
// }

// impl PrettyPrint for UnaryOperator {
//     fn pretty_print(&self) -> String {
//         match self {
//             UnaryOperator::Negative => "-".to_string(),
//             UnaryOperator::Inverse => "!".to_string(),
//             UnaryOperator::Increment => "++".to_string(),
//             UnaryOperator::Decrement => "--".to_string(),
//         }
//     }
// }

// impl PrettyPrint for LogicalOperation {
//     fn pretty_print(&self) -> String {
//         match self {
//             LogicalOperation::And => "&&".to_string(),
//             LogicalOperation::Or => "||".to_string(),
//             LogicalOperation::Equality => "==".to_string(),
//             LogicalOperation::Inequality => "!=".to_string(),
//             LogicalOperation::LessThan => "<".to_string(),
//             LogicalOperation::LessOrEqualThan => "<=".to_string(),
//             LogicalOperation::GreaterThan => ">".to_string(),
//             LogicalOperation::GreaterOrEqualThan => ">=".to_string(),
//         }
//     }
// }

// impl PrettyPrint for BinaryOperator {
//     fn pretty_print(&self) -> String {
//         match self {
//             BinaryOperator::Addition => "+".to_string(),
//             BinaryOperator::Subtraction => "-".to_string(),
//             BinaryOperator::Multiplication => "*".to_string(),
//             BinaryOperator::Division => "/".to_string(),
//             BinaryOperator::Modulo => "%".to_string(),
//             BinaryOperator::Power => "^".to_string(),
//             BinaryOperator::ShiftLeft => "<<".to_string(),
//             BinaryOperator::ShiftRight => ">>".to_string(),
//             BinaryOperator::LogicalOperation(x) => x.pretty_print(),
//         }
//     }
// }

// impl PrettyPrint for Assignment {
//     fn pretty_print(&self) -> String {
//         match self {
//             Assignment::Assign => ":=".to_string(),
//             Assignment::AddAssign => "+=".to_string(),
//             Assignment::SubAssign => "-=".to_string(),
//             Assignment::MulAssign => "*=".to_string(),
//             Assignment::DivAssign => "/=".to_string(),
//         }
//     }
// }

// impl PrettyPrint for Expression {
//     fn pretty_print(&self) -> String {
//         match self {
//             Expression::Value(t, value) => {
//                 let formatted_value = match t {
//                     Type::String => format!("\"{}\"", value),
//                     _ => value.to_string(),
//                 };
//                 return format!("{}({})", t.pretty_print(), formatted_value);
//             },
//             Expression::UnaryOperation(op, expr) => {
//                 match op.notation() {
//                     Notation::Prefix => format!("{}{}", op.pretty_print(), expr.pretty_print()),
//                     Notation::Postfix => format!("{}{}", expr.pretty_print(), op.pretty_print()),
//                     Notation::Infix => "_".to_string(),
//                 }
//             },
//             Expression::BinaryOperation(op, x , y ) => {
//                 format!("{} {} {}", x.pretty_print(), op.pretty_print(), y.pretty_print())
//             },
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn pretty_print_unary() {
//         let unary: UnaryOperator = UnaryOperator::Negative;
//         assert_eq!(unary.pretty_print(), String::from("-"));
//     }
// }