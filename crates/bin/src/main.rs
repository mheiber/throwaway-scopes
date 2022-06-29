use checker::{self, data::*};

struct Example {
    expr: WithPos<Expr>,
    exp_ty: Ty,
    target: WithPos<Expr>,
}

fn example() -> Example {
    let num_with_pos = Expr::Number(20).positioned(Pos(1, 4));
    let target = num_with_pos.clone();
    let fun: Fun = Fun {
        param: Var("x".to_string()),
        param_ty: Ty::Str,
        ret_ty: Ty::Num,
        body: Box::new(
            Expr::If {
                cond: Box::new(Expr::Var(Var("x".to_string())).positioned(Pos(0, 3))),
                true_branch: Box::new(num_with_pos),
                false_branch: Box::new(Expr::False.positioned(Pos(2, 4))),
            }
            .positioned(Pos(0, 0)),
        ),
    };
    let exp_ty = fun.ret_ty.clone();
    let expr = Expr::Fun(fun).positioned(Pos(0, 0));
    Example {
        expr,
        exp_ty,
        target,
    }
}

fn main() {
    let Example {
        expr,
        exp_ty,
        target,
    } = example();
    let diags = checker::check(&expr, exp_ty.clone());
    if diags.is_empty() {
        println!("no type errors found");
    } else {
        println!("type errors:");
        for diag in diags {
            println!("   {}", diag);
        }
    }

    let ty_of = checker::type_of(&expr, exp_ty, target.clone());
    println!(
        "\n type of {} at {} is {:?}",
        target.item, target.pos, ty_of
    );
}
