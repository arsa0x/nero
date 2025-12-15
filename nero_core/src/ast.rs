#[derive(Debug, PartialEq, Clone)]
pub struct Req {
    pub label: Option<String>,
    pub method: String,
    pub url: Expr,
    pub headers: Vec<(String, Expr)>,
    pub query: Vec<(String, Expr)>,
    pub body: Option<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Identifier(String),
    Number(i64),
    String(Vec<StringPart>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum StringPart {
    Text(String),
    Expression(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Assignment { name: String, value: Expr },
    Request(Req),
}
