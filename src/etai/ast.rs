pub enum NodeType {
    Program,
    NumericLiteral,
    Identifier,
    BinaryExpr,
}

pub trait Stmt {
    fn kind(&self) -> NodeType;
    fn new() -> Self where Self: Sized;
}

pub struct Program {
    pub kind: NodeType,
    pub body: Vec<Box<dyn Stmt>>,
}
impl Stmt for Program {
    fn kind(&self) -> NodeType {
        NodeType::Program
    }
    fn new() -> Program {
        Program {kind: NodeType::Program, body: Vec::new()}
    }
}

pub trait Expr {
    fn kind(&self) -> NodeType;
}

pub struct BinaryExpr {
    kind: NodeType,
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
    operator: String,
}
impl Expr for BinaryExpr {
    fn kind(&self) -> NodeType {
        NodeType::BinaryExpr
    }
}
impl BinaryExpr {
    fn new(left: Box<dyn Expr>, right: Box<dyn Expr>, operator: String) -> BinaryExpr {
        BinaryExpr {kind: NodeType::BinaryExpr, left: left, right: right, operator: operator}
    }
}

pub struct Identifier {
    kind: NodeType,
    symbol: String,
}
impl Expr for Identifier {
    fn kind(&self) -> NodeType {
        NodeType::Identifier
    }
}
impl Identifier {
    fn new(symbol: String) -> Identifier {
        Identifier { kind: NodeType::Identifier, symbol: symbol}
    }
}

pub struct NumericLiteral {
    kind: NodeType,
    value: f64,
}
impl Expr for NumericLiteral {
    fn kind(&self) -> NodeType {
        NodeType::Identifier
    }
}
impl NumericLiteral {
    fn new(value: f64) -> NumericLiteral {
        NumericLiteral { kind: NodeType::NumericLiteral, value: value}
    }
}