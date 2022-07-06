use std::error::Error;
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Ty {
    Bool,
    Num,
    Str,
    Union(Box<Ty>, Box<Ty>),
    ProducedDiagnostic,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Pos(pub u32, pub u32);

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Var(pub String);

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Fun {
    pub param: Var,
    pub param_ty: Ty,
    pub ret_ty: Ty,
    pub body: Box<WithPos<Expr>>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WithPos<T> {
    pub item: T,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum DiagnosticKind {
    ExpectedTy { expected: Ty, got: Ty },
    UnboundVar { name: String },
}

#[derive(Debug)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub expr_with_pos: WithPos<Expr>,
}

#[derive(Debug)]
pub struct FoundTy(pub Ty);

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Assign(Box<Var>, Box<WithPos<Expr>>),
    If {
        cond: Box<WithPos<Expr>>,
        true_branch: Box<WithPos<Expr>>,
        false_branch: Box<WithPos<Expr>>,
    },
    True,
    False,
    Number(i32),
    Var(Var),
    Fun(Fun),
    Block {
        non_empty_exprs: Vec<WithPos<Expr>>,
    },
}

impl Expr {
    pub fn positioned(self, pos: Pos) -> WithPos<Expr> {
        WithPos { item: self, pos }
    }
}

impl Error for FoundTy {}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Expr::Assign(_, _) => "Assignment".to_string(),
            Expr::If {
                cond: _,
                true_branch: _,
                false_branch: _,
            } => "If".to_string(),
            Expr::True => "True".to_string(),
            Expr::False => "False".to_string(),
            Expr::Number(n) => format!("{}", n),
            Expr::Var(Var(name)) => format!("'{}'", name),
            Expr::Fun(_) => "fun".to_string(),
            Expr::Block { non_empty_exprs: _ } => "block".to_string(),
        };
        write!(f, "{}", s)
    }
}

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ty::Bool => write!(f, "bool"),
            Ty::Num => write!(f, "num"),
            Ty::Str => write!(f, "str"),
            Ty::ProducedDiagnostic => write!(f, "ERROR"),
            Ty::Union(ty1, ty2) => write!(f, "({} | {})", *ty1, *ty2),

        }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {} col {}", self.0 + 1, self.1)
    }
}

impl Display for DiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticKind::ExpectedTy { expected, got } => {
                write!(f, "Expected {:?}, got {:?}", expected, got)
            }
            DiagnosticKind::UnboundVar { name } => write!(f, "unbound var {}", name),
        }
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} for expression {}",
            self.expr_with_pos.pos, self.kind, self.expr_with_pos.item
        )
    }
}

impl Display for FoundTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
