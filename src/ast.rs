struct Block {
    statements: Vec<Statement>,
}

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

struct AssignmentStatement {
    assignee: Expression,
    operator: AssignmentOperator,
    expression: Expression,
}

enum UpdateOperator {
    Increment, // ++
    Decrement, // --
}

struct UpdateStatement {
    identifier: String,
    operator: UpdateOperator,
}

struct IfStatement {
    test: Expression,
    block: Block,
}
// TODO: how to do the else branches?

struct WhileStatement {
    test: Expression,
    block: Block,
}

struct EachStatement {
    left: String,
    right: Expression,
    block: Block,
}

struct VariableDeclarationStatement {
    identifier: String,
    type_specifier: Option<TypeSpecifier>,
    expression: Expression,
}

struct Parameter {
    identifier: String,
    type_specifier: TypeSpecifier,
}

struct FunctionDeclarationStatement {
    identifier: String,
    parameters: Vec<Parameter>,
    return_type: Option<TypeSpecifier>,
    block: Block,
}

struct ReturnStatement {
    expression: Expression,
}

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

struct BinaryExpression {
    left: Expression,
    operator: BinaryOperator,
    right: Expression,
}

enum UnaryOperator {
    Not,    // !
    Negate, // -
}

struct UnaryExpression {
    operator: UnaryOperator,
    right: Expression,
}

struct IdentifierExpression {
    identifier: String,
}

struct FunctionCallExpression {
    callee: Expression,
    arguments: Vec<Expression>,
}

struct IndexExpression {
    collection: Expression,
    index: Expression,
}

enum Expression {
    Binary(Box<BinaryExpression>),
    Unary(Box<UnaryExpression>),
    Identifier(IdentifierExpression),
    FunctionCall(Box<FunctionCallExpression>),
    Index(Box<IndexExpression>),
    Literal(LiteralExpression),
    ParenthesisedExpression(Box<Expression>),
}

enum LiteralExpression {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Expression>),
    Null,
}

enum TypeSpecifier {}
