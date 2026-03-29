// ------------------------------------------------------------
// OGMA — AST Officiel 0.3.11
// ------------------------------------------------------------
// Cet AST représente fidèlement la grammaire officielle :
// - Modules
// - Imports
// - Déclarations
// - Fonctions
// - Objets
// - Blocs
// - Expressions
// - Chemins
// - Appels
// - Dialectes
// ------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ModuleFile {
    pub name: String,
    pub imports: Vec<Import>,
    pub declarations: Vec<Decl>,
}

// ------------------------------------------------------------
// IMPORTS
// ------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Import {
    pub module: String,
    pub alias: Option<String>,
}

// ------------------------------------------------------------
// DÉCLARATIONS
// ------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Decl {
    Variable(VarDecl),
    Function(FuncDecl),
    Object(ObjectDecl),
    Module(NestedModuleDecl),
}

// x: int = expr
#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: String,
    pub type_hint: TypeRef,
    pub value: Expr,
}

// nom: fn(a: int, b: int) -> int { ... }
#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<TypeRef>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub type_hint: TypeRef,
}

// object { ... }
#[derive(Debug, Clone)]
pub struct ObjectDecl {
    pub name: String,
    pub fields: Vec<ObjectField>,
}

#[derive(Debug, Clone)]
pub struct ObjectField {
    pub mutable: bool,
    pub name: String,
    pub type_hint: TypeRef,
    pub value: Expr,
}

// module math { ... }
#[derive(Debug, Clone)]
pub struct NestedModuleDecl {
    pub name: String,
    pub declarations: Vec<Decl>,
}

// ------------------------------------------------------------
// TYPES
// ------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum TypeRef {
    Simple(String), // int, decimal, string, bool, char, Prix...
    Ognum { precision: i32, scale: i32 },
}

// ------------------------------------------------------------
// BLOCS
// ------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
    pub last_expr: Option<Expr>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Decl(VarDecl),
    Expr(Box<Expr>),
}

// ------------------------------------------------------------
// EXPRESSIONS
// ------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Identifier(String),
    Path(Path),
    Call(Call),
    MethodCall(MethodCall),
    ObjectLiteral(ObjectLiteral),
    Block(Box<Block>),
    ExplicitBlock(Box<Block>),
    Dialect(DialectBlock),
}

// a.b.c
#[derive(Debug, Clone)]
pub struct Path {
    pub segments: Vec<String>,
}

// f(arg1, arg2)
#[derive(Debug, Clone)]
pub struct Call {
    pub function: Box<Expr>,
    pub args: Vec<Expr>,
}

// a.b.c(arg1, arg2)
#[derive(Debug, Clone)]
pub struct MethodCall {
    pub target: Box<Expr>,
    pub method: String,
    pub args: Vec<Expr>,
}

// object { ... } comme expression
#[derive(Debug, Clone)]
pub struct ObjectLiteral {
    pub fields: Vec<ObjectField>,
}

// [ ... ] dialecte
#[derive(Debug, Clone)]
pub struct DialectBlock {
    pub lines: Vec<String>,
}

// ------------------------------------------------------------
// LITTÉRAUX
// ------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Decimal(f64),
    Bool(bool),
    String(String),
    Char(char),
    Ognum(String), // littéral brut, typé plus tard
}
