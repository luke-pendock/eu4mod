
// Ignore increment and decrement as we are producing a functional language.
#[derive(PartialEq, Debug, Clone)]
pub enum UnaryOperator {
    Not,
    Negative,
}

impl UnaryOperator {
    pub fn as_string(&self) -> &'static str {
        use UnaryOperator::*;
        match self {
            Not => "!",
            Negative => "-",
        }
    }
}

// Ignore Xor and other bitwise operators due to limited use-cases
#[derive(PartialEq, Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    And,
    Or,
    ShiftLeft,
    ShiftRight,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl BinaryOperator {
    pub fn is_comparison(&self) -> bool {
        use BinaryOperator::*;
        match self {
            Equal
            | NotEqual
            | Less
            | LessEqual
            | Greater
            | GreaterEqual => true,
            _ => false,
        }
    }

    pub fn as_string(&self) -> &'static str {
        use BinaryOperator::*;
        match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            Pow => "**",
            And => "&&",
            Or => "||",
            ShiftLeft => "<<",
            ShiftRight => ">>",
            Equal => "==",
            NotEqual => "!=",
            Less => "<",
            LessEqual => "<=",
            Greater => ">",
            GreaterEqual => ">=",
        }
    }

    // // Only applies to Real numbers
    // pub fn is_commutative(&self) -> bool {
    //     use BinaryOperator::*;
    //     match self {
    //         Add |
    //         Mul |
    //         And |
    //         Or => true,
    //         _ => false,
    //     }
    // }
}

