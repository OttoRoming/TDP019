use crate::util::Region;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub test: Expression,
    pub block: Block,
    pub branch: Option<IfBranch>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IfBranch {
    Elif(ElifPart),
    Else(ElsePart),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElifPart {
    pub test: Expression,
    pub block: Block,
    pub branch: Box<Option<IfBranch>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElsePart {
    pub block: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement {
    pub test: Expression,
    pub block: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EachStatement {
    pub left: String,
    pub right: Expression,
    pub block: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarationStatement {
    pub identifier: String,
    pub type_specifier: Option<TypeSpecifier>,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub identifier: String,
    pub type_specifier: TypeSpecifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclarationStatement {
    pub identifier: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeSpecifier>,
    pub block: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub expression: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Throw {
    pub message: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TryCatch {
    pub try_block: Block,
    pub exception_identifier: Option<String>,
    pub catch_block: Block,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum StatementValue {
    Block(Block),
    If(IfStatement),
    While(WhileStatement),
    Each(EachStatement),
    VariableDeclaration(VariableDeclarationStatement),
    FunctionDeclaration(FunctionDeclarationStatement),
    Return(ReturnStatement),
    Throw(Throw),
    TryCatch(TryCatch),
    Continue,
    Break,
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    pub value: StatementValue,
    pub region: Region,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssignmentOperator {
    Equals,   // =
    Add,      // +=
    Subtract, // -=
    Multiply, // *=
    Divide,   // /=
    Modulo,   // %=
    And,      // &=
    Or,       // |=
    Append,   // <-
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentExpression {
    pub assignee: Expression,
    pub operator: AssignmentOperator,
    pub right: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UpdateOperator {
    Increment, // ++
    Decrement, // --
}

#[derive(Debug, PartialEq, Clone)]
pub struct UpdateExpression {
    pub updatee: Expression,
    pub operator: UpdateOperator,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub left: Expression,
    pub operator: BinaryOperator,
    pub right: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Not,         // !
    Negate,      // -
    Dereference, // *
    Reference,   // &
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub right: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdentifierExpression {
    pub identifier: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCallExpression {
    pub callee: Expression,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexExpression {
    pub collection: Expression,
    pub index: Expression,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum ExpressionValue {
    Assign(Box<AssignmentExpression>),
    Update(Box<UpdateExpression>),
    Binary(Box<BinaryExpression>),
    Unary(Box<UnaryExpression>),
    Identifier(IdentifierExpression),
    FunctionCall(Box<FunctionCallExpression>),
    Index(Box<IndexExpression>),
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub value: ExpressionValue,
    pub region: Region,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeSpecifier {
    Int,
    Float,
    String,
    Bool,
    List(Box<TypeSpecifier>),
    Ref(Box<TypeSpecifier>),
}
