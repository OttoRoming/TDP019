#[derive(Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum AssignmentOperator {
    Equals,   // =
    Add,      // +=
    Subtract, // -=
    Multiply, // *=
    Divide,   // /=
    Modulo,   // %=
    And,      // &=
    Or,       // |=
}

#[derive(Debug, PartialEq)]
pub struct AssignmentStatement {
    pub assignee: Expression,
    pub operator: AssignmentOperator,
    pub expression: Expression,
}

#[derive(Debug, PartialEq)]
pub enum UpdateOperator {
    Increment, // ++
    Decrement, // --
}

#[derive(Debug, PartialEq)]
pub struct UpdateStatement {
    pub identifier: String,
    pub operator: UpdateOperator,
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    pub test: Expression,
    pub block: Block,
}
// TODO: how to do the else branches?

#[derive(Debug, PartialEq)]
pub struct WhileStatement {
    pub test: Expression,
    pub block: Block,
}

#[derive(Debug, PartialEq)]
pub struct EachStatement {
    pub left: String,
    pub right: Expression,
    pub block: Block,
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclarationStatement {
    pub identifier: String,
    pub type_specifier: Option<TypeSpecifier>,
    pub expression: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub identifier: String,
    pub type_specifier: TypeSpecifier,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclarationStatement {
    pub identifier: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeSpecifier>,
    pub block: Block,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub expression: Expression,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assignment(AssignmentStatement),
    Update(UpdateStatement),
    If(IfStatement),
    While(WhileStatement),
    Each(EachStatement),
    VariableDeclaration(VariableDeclarationStatement),
    FunctionDeclaration(FunctionDeclarationStatement),
    Return(ReturnStatement),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equals,
    NotEquals,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression {
    pub left: Expression,
    pub operator: BinaryOperator,
    pub right: Expression,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,    // !
    Negate, // -
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub right: Expression,
}

#[derive(Debug, PartialEq)]
pub struct IdentifierExpression {
    pub identifier: String,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCallExpression {
    pub callee: Expression,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct IndexExpression {
    pub collection: Expression,
    pub index: Expression,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Binary(Box<BinaryExpression>),
    Unary(Box<UnaryExpression>),
    Identifier(IdentifierExpression),
    FunctionCall(Box<FunctionCallExpression>),
    Index(Box<IndexExpression>),
    Literal(LiteralExpression),
    Parenthesised(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum LiteralExpression {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Expression>),
    Null,
}

#[derive(Debug, PartialEq)]
pub enum TypeSpecifier {
    Int,
    Float,
    String,
    Bool,
    List(Box<TypeSpecifier>),
    Ref(Box<TypeSpecifier>),
}
