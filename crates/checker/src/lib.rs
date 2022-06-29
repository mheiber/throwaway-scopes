mod ctx;
pub mod data;
use crate::ctx::Ctx;
use crate::data::*;
use anyhow::{self, bail, Context, Result};

pub fn check(e_with_pos: &WithPos<Expr>, ret_ty: Ty) -> Vec<Diagnostic> {
    run_with_mode(e_with_pos, true, None, ret_ty)
        .context("expected diagnostics")
        .unwrap()
}

pub fn type_of(e_with_pos: &WithPos<Expr>, ret_ty: Ty, target: WithPos<Expr>) -> Ty {
    match run_with_mode(e_with_pos, false, Some(target), ret_ty) {
        Err(err) => {
            if let Some(FoundTy(ty)) = err.downcast_ref::<FoundTy>() {
                ty.clone()
            } else {
                panic!("internal error {:?}", err)
            }
        }
        res => panic!("internal error, expected to get type but got {:?}", res),
    }
}

fn run_with_mode(
    e: &WithPos<Expr>,
    diagnose: bool,
    look_for: Option<WithPos<Expr>>,
    exp_ty: Ty,
) -> Result<Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    let mut scopes = Vec::new();
    let mut ctx = Ctx::new(&mut diagnostics, &mut scopes, 0, diagnose, &look_for);
    run(&mut ctx, e, Some(exp_ty))?;
    Ok(diagnostics)
}

pub fn run<'a>(ctx: &mut Ctx<'a>, e_with_pos: &WithPos<Expr>, exp_ty: Option<Ty>) -> Result<Ty> {
    let e = &e_with_pos.item;
    let res = match e {
        Expr::Assign(var, expr) => {
            let name = var.0.clone();
            let ty = run(ctx, expr, exp_ty.clone())?;
            ctx.insert(name, ty.clone());
            Ok(ty)
        }
        Expr::If {
            cond,
            true_branch,
            false_branch,
        } => {
            run(ctx, cond, Some(Ty::Bool))?;
            let t_ty = run(ctx, true_branch, exp_ty.clone())?;
            let false_branch_exp_ty = exp_ty.clone().unwrap_or(t_ty);
            run(ctx, false_branch, Some(false_branch_exp_ty))
        }
        Expr::True | Expr::False => Ok(Ty::Bool),
        Expr::Number(_) => Ok(Ty::Num),
        Expr::Var(Var(name)) => match ctx.get(name) {
            Some(ty) => Ok(ty.clone()),
            None => {
                ctx.add_diagnostic(|| Diagnostic {
                    kind: DiagnosticKind::UnboundVar { name: name.clone() },
                    expr_with_pos: e_with_pos.clone(),
                });
                Ok(Ty::ProducedDiagnostic)
            }
        },
        Expr::Block { non_empty_exprs } => non_empty_exprs
            .iter()
            .map(|expr| run(ctx, expr, exp_ty.clone()))
            .last()
            .unwrap(),
        Expr::Fun(Fun {
            param,
            param_ty,
            ret_ty,
            body,
        }) => ctx.enter(&mut |ctx| {
            ctx.insert(param.0.clone(), param_ty.clone());
            run(ctx, body, Some(ret_ty.clone()))?;
            Ok(ret_ty.clone())
        }),
    };

    if ctx.looking_for(e_with_pos) {
        let ty = res.context("internal error, expected type")?;
        bail!(FoundTy(ty))
    }

    check_for_ty_mismatch(ctx, &res, exp_ty, e_with_pos.clone());
    res
}

fn check_for_ty_mismatch(
    ctx: &mut Ctx<'_>,
    res: &Result<Ty>,
    exp_ty: Option<Ty>,
    expr_with_pos: WithPos<Expr>,
) -> () {
    fn has_ty_mismatch(t1: &Ty, t2: &Ty) -> bool {
        match (t1, t2) {
            (Ty::ProducedDiagnostic, _) => false,
            (_, Ty::ProducedDiagnostic) => false,
            _ => t1 != t2,
        }
    }

    match (res, exp_ty) {
        (Ok(ty), Some(exp_ty)) if has_ty_mismatch(ty, &exp_ty) => {
            ctx.add_diagnostic(|| Diagnostic {
                kind: DiagnosticKind::ExpectedTy {
                    expected: exp_ty,
                    got: ty.clone(),
                },
                expr_with_pos,
            });
        }
        _ => (),
    }
}
