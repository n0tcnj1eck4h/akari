#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Equal,
    Assign,
    //////
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    NotEqual,
    //////
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
    //////
    BinaryAnd,
    BinaryOr,
    BinaryXor,
    BinaryNot,
    BinaryLeft,
    BinaryRight,
    //////
    LogicAnd,
    LogicOr,
    LogicNot,
}

#[rustfmt::skip]
impl Operator {
    pub fn get_precedence(&self) -> i32 {
        match self {
           Operator::Power          => 200,
           Operator::Multiply       => 100,
           Operator::Divide         => 100,
           Operator::Modulo         => 100,
           Operator::Add            => 80,
           Operator::Subtract       => 80,
           Operator::BinaryRight    => 60,
           Operator::BinaryLeft     => 60,
           Operator::Less           => 40,
           Operator::LessOrEqual    => 40,
           Operator::Greater        => 40,
           Operator::GreaterOrEqual => 40,
           Operator::BinaryAnd      => 36,
           Operator::BinaryXor      => 33,
           Operator::BinaryOr       => 30,
           Operator::Equal          => 20,
           Operator::NotEqual       => 20,
           Operator::LogicAnd       => 15,
           Operator::LogicOr        => 10,
           Operator::Assign         => 5,
           Operator::BinaryNot      => -1,
           Operator::LogicNot       => -1,
        }
    }
}
