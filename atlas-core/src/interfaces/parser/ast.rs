use core::fmt;
use std::fmt::write;

use crate::{utils::span::WithSpan, prelude::visitor::Visitor, Token};

use super::node::Node;

/// Placeholder
pub type AbstractSyntaxTree = Vec<WithSpan<Box<Expression>>>;

/// Literal
#[derive(Debug, Clone)]
pub enum Literal {
    /// Integer literal
    Integer(i64),
    /// Float literal
    Float(f64),
    /// String literal
    String(String),
    /// Boolean literal
    Bool(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(fl) => write!(f, "{}", fl),
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
    Return(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VariableDeclaration(v) => write!(f, "{};", v),
            Self::Expression(e) => write!(f, "{};", e),
            Self::Return(e) => write!(f, "return {};", e),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Integer,
    Float,
    String,
    Bool,
    Void,
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Function(Vec<(String, Type)>, Box<Type>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer => write!(f, "i64"),
            Self::Float => write!(f, "f64"),
            Self::String => write!(f, "string"),
            Self::Bool => write!(f, "bool"),
            Self::Void => write!(f, "void"),
            Self::List(t) => write!(f, "List[{}]", t),
            Self::Map(k, v) => write!(f, "Map[{}, {}]", k, v),
            Self::Function(args, ret) => write!(f, "({}) -> {}", args.iter().map(|(s, t)| format!("{}: {}", s, t)).collect::<Vec<String>>().join(", "), ret),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_type_display() {
        assert_eq!(Type::Integer.to_string(), "i64");
        assert_eq!(Type::Float.to_string(), "f64");
        assert_eq!(Type::String.to_string(), "string");
        assert_eq!(Type::Bool.to_string(), "bool");
        assert_eq!(Type::Void.to_string(), "void");
        assert_eq!(Type::List(Box::new(Type::Integer)).to_string(), "List[i64]");
        assert_eq!(Type::Map(Box::new(Type::Integer), Box::new(Type::String)).to_string(), "Map[i64, string]");
        assert_eq!(Type::Function(vec![(String::from("abc"), Type::Integer), (String::from("efg"), Type::Float)], Box::new(Type::Bool)).to_string(), "(abc: i64, efg: f64) -> bool");
    }
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub t: Type,
    pub mutable: bool,
    pub value: Option<WithSpan<Box<Expression>>>,
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.is_some() {
            write!(f, "let {}{}: {} = {}\n", if self.mutable { "mut " } else { "" }, self.name, self.t, self.value.clone().unwrap().value)
        } else {
            write!(f, "let {}{}: {}\n", if self.mutable { "mut " } else { "" }, self.name, self.t)
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionExpression {
    pub args: Vec<(String, Type)>,
    pub body: WithSpan<Box<Expression>>,
}

impl fmt::Display for FunctionExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body.value)
    }
}

/// Identifier
#[derive(Debug, Clone)]
pub struct IdentifierNode {
    /// Name of the identifier
    pub name: String,
}

impl fmt::Display for IdentifierNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Node for IdentifierNode {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_identifier(self);
    }
}

/// Binary expression 
#[derive(Debug, Clone)]
pub struct BinaryExpression {
    /// Left member of the binary expression
    /// Can be any expression, including another binary expression
    pub left: WithSpan<Box<Expression>>,
    /// Operator of the binary expression see `BinaryOperator`
    pub operator: BinaryOperator,
    /// Right member of the binary expression
    /// Can be any expression, including another binary expression
    pub right: WithSpan<Box<Expression>>,
}

impl Node for BinaryExpression {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_binary_expression(self);
    }
}

impl fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left.value, self.operator, self.right.value)
    }
}

/// Binary operator
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    /// Addition (`+`)
    OpAdd,
    /// Subtraction (`-`)
    OpSub,
    /// Multiplication (`*`)
    OpMul,
    /// Division (`/`)
    OpDiv,
    /// Modulo (`%`)
    OpMod,
    /// Power (`^`)
    OpPow,
    /*OpEq,
    OpNe,
    OpLt,
    OpLe,
    OpGt,
    OpGe,
    OpAnd,
    OpOr,
    OpXor,
    OpShl,
    OpShr,
    OpBitAnd,
    OpBitOr,
    OpBitXor,*/
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpAdd => write!(f, "+"),
            Self::OpSub => write!(f, "-"),
            Self::OpMul => write!(f, "*"),
            Self::OpDiv => write!(f, "/"),
            Self::OpMod => write!(f, "%"),
            Self::OpPow => write!(f, "^"),
        }
    }
}

impl From<&Token> for Option<BinaryOperator> {
    fn from(value: &Token) -> Self {
        match value {
            Token::OpAdd => Some(BinaryOperator::OpAdd),
            Token::OpSub => Some(BinaryOperator::OpSub),
            Token::OpMul => Some(BinaryOperator::OpMul),
            Token::OpDiv => Some(BinaryOperator::OpDiv),
            Token::OpMod => Some(BinaryOperator::OpMod),
            Token::OpPow => Some(BinaryOperator::OpPow),
            /*Token::OpEq => Some(BinaryOperator::OpEq),
            Token::OpNe => Some(BinaryOperator::OpNe),
            Token::OpLt => Some(BinaryOperator::OpLt),
            Token::OpLe => Some(BinaryOperator::OpLe),
            Token::OpGt => Some(BinaryOperator::OpGt),
            Token::OpGe => Some(BinaryOperator::OpGe),
            Token::OpAnd => Some(BinaryOperator::OpAnd),
            Token::OpOr => Some(BinaryOperator::OpOr),*/
            _ => None,
        }
    }
}

/// Unary expression
#[derive(Debug, Clone)]
pub struct UnaryExpression {
    /// Operator of the unary expression, see `UnaryOperator`
    pub operator: Option<UnaryOperator>,
    /// Expression of the unary expression
    /// Can be any expression, including another unary expression
    pub expression: WithSpan<Box<Expression>>,
}

impl fmt::Display for UnaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.operator.is_some() {
            write!(f, "{} {}", self.operator.clone().unwrap(), self.expression.value)
        } else {
            write!(f, "{}", self.expression.value)
        }
    }
}

impl Node for UnaryExpression {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_unary_expression(self);
    }
}

/// Unary operator
#[derive(Debug, Clone)]
pub enum UnaryOperator {
    /// Addition (`+`)
    OpAdd,
    /// Subtraction (`-`)
    OpSub,
    /// Not (`!`)
    OpNot,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpAdd => write!(f, "+"),
            Self::OpSub => write!(f, "-"),
            Self::OpNot => write!(f, "!"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IfElseNode {
    pub condition: WithSpan<Box<Expression>>,
    pub if_body: WithSpan<Box<Expression>>,
    pub else_body: Option<WithSpan<Box<Expression>>>,
}

impl fmt::Display for IfElseNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(e) = &self.else_body {
            write!(f, "if {}then\n\t{}else\n\t{}", self.condition.value, self.if_body.value, e.value)
        } else {
            write!(f, "if {} then\n\t{}", self.condition.value, self.if_body.value)
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<WithSpan<Box<Expression>>>,
}
impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.args.iter().map(|a| a.value.to_string()).collect::<Vec<String>>().join(", "))
    }
}

#[derive(Debug, Clone)]
pub struct DoExpression {
    pub body: Vec<WithSpan<Box<Expression>>>,
}
impl fmt::Display for DoExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "do\n\t{}", self.body.iter().map(|a| a.value.to_string()).collect::<Vec<String>>().join("\n\t"))
    }
}

/// Expression
#[derive(Debug, Clone)]
pub enum Expression {
    /// Contains the `Literal` enum
    Literal(Literal),
    /// Contains the `IdentifierNode` struct
    Identifier(IdentifierNode),
    /// Contains the `BinaryExpression` struct
    BinaryExpression(BinaryExpression),
    /// Contains the `UnaryExpression` struct
    UnaryExpression(UnaryExpression),
    IfElseNode(IfElseNode),
    FunctionExpression(FunctionExpression),
    VariableDeclaration(VariableDeclaration),
    FunctionCall(FunctionCall),
    DoExpression(DoExpression),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(l) => write!(f, "{}", l),
            Self::Identifier(i) => write!(f, "{}", i),
            Self::BinaryExpression(b) => write!(f, "{}", b),
            Self::UnaryExpression(u) => write!(f, "{}", u),
            Self::IfElseNode(i) => write!(f, "{}", i),
            Self::FunctionExpression(fun) => write!(f, "{}", fun),
            Self::VariableDeclaration(v) => write!(f, "{}", v),
            Self::FunctionCall(fun) => write!(f, "{}", fun),
            Self::DoExpression(d) => write!(f, "{}", d),
        }
    }
}

impl Node for Expression {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        match self {
            Self::Identifier(i) => {
                visitor.visit_identifier(i);
            },
            Self::BinaryExpression(b) => {
                visitor.visit_binary_expression(b);
            },
            _ => ()
        }
    }
}