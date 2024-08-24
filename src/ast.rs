use crate::token::Operator;

#[derive(Debug)]
pub struct Module {
    pub imports: Vec<Import>,
    pub function_declarations: Vec<FunctionDeclaration>,
    pub function_definitions: Vec<FunctionDefinition>,
    pub type_definitions: Vec<Composite>,
    pub globals: Vec<GlobalVariableDefintion>,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub body: Statement,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub calling_convention: Option<String>,
    pub return_type: Option<String>,
}

#[derive(Debug)]
pub struct GlobalVariableDefintion {
    pub datatype: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Import {
    pub path: Vec<String>,
}

#[derive(Debug)]
pub struct Parameter {
    pub param_type: String,
    pub name: String,
}

#[derive(Debug)]
pub enum Statement {
    Block(Vec<Statement>),
    Conditional(Expression, Box<Statement>, Option<Box<Statement>>),
    LocalVar(String, Option<String>, Option<Expression>),
    Loop(Expression, Box<Statement>),
    Assignment(String, Expression),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Expression {
    IntegerLiteral(i128),
    FloatingPointLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Identifier(String),
    BinaryOperation(Box<Expression>, Operator, Box<Expression>),
    UnaryOperation(Operator, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
}

#[derive(Debug)]
pub struct CompositeField {
    pub name: String,
    pub datatype: String,
}

#[derive(Debug)]
pub struct Composite {
    pub name: String,
    pub fields: Vec<CompositeField>,
}
