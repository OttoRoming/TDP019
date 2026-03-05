#[allow(unused)]
struct Block {
    statements: Vec<Statement>,
}

#[allow(unused)]
enum AssignmentOperator {
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
struct AssignmentStatement {
    assignee: Expression,
    operator: AssignmentOperator,
    expression: Expression,
}

#[allow(unused)]
enum UpdateOperator {
    Increment, // ++
    Decrement, // --
}

#[allow(unused)]
struct UpdateStatement {
    identifier: String,
    operator: UpdateOperator,
}

#[allow(unused)]
struct IfStatement {
    test: Expression,
    block: Block,
}
// TODO: how to do the else branches?

#[allow(unused)]
struct WhileStatement {
    test: Expression,
    block: Block,
}

#[allow(unused)]
struct EachStatement {
    left: String,
    right: Expression,
    block: Block,
}

#[allow(unused)]
struct VariableDeclarationStatement {
    identifier: String,
    type_specifier: Option<TypeSpecifier>,
    expression: Expression,
}

#[allow(unused)]
struct Parameter {
    identifier: String,
    type_specifier: TypeSpecifier,
}

#[allow(unused)]
struct FunctionDeclarationStatement {
    identifier: String,
    parameters: Vec<Parameter>,
    return_type: Option<TypeSpecifier>,
    block: Block,
}

#[allow(unused)]
struct ReturnStatement {
    expression: Expression,
}

#[allow(unused)]
enum Statement {
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
enum BinaryOperator {
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
struct BinaryExpression {
    left: Expression,
    operator: BinaryOperator,
    right: Expression,
}

#[allow(unused)]
enum UnaryOperator {
    Not,    // !
    Negate, // -
}

#[allow(unused)]
struct UnaryExpression {
    operator: UnaryOperator,
    right: Expression,
}

#[allow(unused)]
struct IdentifierExpression {
    identifier: String,
}

#[allow(unused)]
struct FunctionCallExpression {
    callee: Expression,
    arguments: Vec<Expression>,
}

#[allow(unused)]
struct IndexExpression {
    collection: Expression,
    index: Expression,
}

#[allow(unused)]
enum Expression {
    Binary(Box<BinaryExpression>),
    Unary(Box<UnaryExpression>),
    Identifier(IdentifierExpression),
    FunctionCall(Box<FunctionCallExpression>),
    Index(Box<IndexExpression>),
    Literal(LiteralExpression),
    Parenthesised(Box<Expression>),
}

#[allow(unused)]
enum LiteralExpression {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Expression>),
    Null,
}

#[allow(unused)]
enum TypeSpecifier {
    Int,
    Float,
    String,
    Bool,
    List(Box<TypeSpecifier>),
    Ref(Box<TypeSpecifier>),
}
