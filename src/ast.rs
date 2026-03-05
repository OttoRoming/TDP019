#[allow(unused)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[allow(unused)]
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

#[allow(unused)]
pub struct AssignmentStatement {
    pub assignee: Expression,
    pub operator: AssignmentOperator,
    pub expression: Expression,
}

#[allow(unused)]
pub enum UpdateOperator {
    Increment, // ++
    Decrement, // --
}

#[allow(unused)]
pub struct UpdateStatement {
    pub identifier: String,
    pub operator: UpdateOperator,
}

#[allow(unused)]
pub struct IfStatement {
    pub test: Expression,
    pub block: Block,
}
// TODO: how to do the else branches?

#[allow(unused)]
pub struct WhileStatement {
    pub test: Expression,
    pub block: Block,
}

#[allow(unused)]
pub struct EachStatement {
    pub left: String,
    pub right: Expression,
    pub block: Block,
}

#[allow(unused)]
pub struct VariableDeclarationStatement {
    pub identifier: String,
    pub type_specifier: Option<TypeSpecifier>,
    pub expression: Expression,
}

#[allow(unused)]
pub struct Parameter {
    pub identifier: String,
    pub type_specifier: TypeSpecifier,
}

#[allow(unused)]
pub struct FunctionDeclarationStatement {
    pub identifier: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeSpecifier>,
    pub block: Block,
}

#[allow(unused)]
pub struct ReturnStatement {
    pub expression: Expression,
}

#[allow(unused)]
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

#[allow(unused)]
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

#[allow(unused)]
pub struct BinaryExpression {
    pub left: Expression,
    pub operator: BinaryOperator,
    pub right: Expression,
}

#[allow(unused)]
pub enum UnaryOperator {
    Not,    // !
    Negate, // -
}

#[allow(unused)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub right: Expression,
}

#[allow(unused)]
pub struct IdentifierExpression {
    pub identifier: String,
}

#[allow(unused)]
pub struct FunctionCallExpression {
    pub callee: Expression,
    pub arguments: Vec<Expression>,
}

#[allow(unused)]
pub struct IndexExpression {
    pub collection: Expression,
    pub index: Expression,
}

#[allow(unused)]
pub enum Expression {
    Binary(Box<BinaryExpression>),
    Unary(Box<UnaryExpression>),
    Identifier(IdentifierExpression),
    FunctionCall(Box<FunctionCallExpression>),
    Index(Box<IndexExpression>),
    Literal(LiteralExpression),
    Parenthesised(Box<Expression>),
}

#[allow(unused)]
pub enum LiteralExpression {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Expression>),
    Null,
}

#[allow(unused)]
pub enum TypeSpecifier {
    Int,
    Float,
    String,
    Bool,
    List(Box<TypeSpecifier>),
    Ref(Box<TypeSpecifier>),
}
