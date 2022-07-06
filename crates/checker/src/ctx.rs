use crate::data::*;
use crate::subtype;
use std::collections::HashMap;

type Scope = HashMap<String, Ty>;

#[derive(Debug)]
pub struct Ctx<'a> {
    index: usize,
    scopes: &'a mut Vec<Scope>,
    diagnostics: &'a mut Vec<Diagnostic>,
    diagnose: bool,
    look_for: &'a Option<WithPos<Expr>>,
}

#[derive(Debug)]
pub struct RootCtx {
    scopes: Vec<Scope>,
    diagnostics: Vec<Diagnostic>,
    diagnose: bool,
    look_for: Option<WithPos<Expr>>,
}

impl RootCtx {
    pub fn new(diagnose: bool, look_for: Option<WithPos<Expr>>) -> Self {
        RootCtx {
            scopes: Default::default(),
            diagnostics: Default::default(),
            diagnose,
            look_for,
        }
    }
    pub fn enter(&mut self) -> Ctx {
        self.scopes.push(Default::default());
        Ctx {
            index: 0,
            scopes: &mut self.scopes,
            diagnostics: &mut self.diagnostics,
            diagnose: self.diagnose,
            look_for: &mut self.look_for,
        }
    }
    pub fn take_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}

impl<'a> Ctx<'a> {
    pub fn enter(&mut self) -> Ctx {
        self.scopes.push(Default::default());
        Ctx {
            index: self.index + 1,
            scopes: self.scopes,
            diagnostics: self.diagnostics,
            diagnose: self.diagnose,
            look_for: self.look_for,
        }
    }
    pub fn get(&self, key: &str) -> Option<Ty> {
        self.scopes.iter().rev().find_map(|s| match s.get(key) {
            Some(ty) => Some((*ty).clone()),
            None => None
        })
    }
    pub fn insert(&mut self, key: String, val: Ty) {
        let g = &mut self.scopes[self.index];
        g.insert(key, val);
    }
    pub fn join<'b, T>(&mut self, it: impl Iterator<Item=T>, f: impl Fn(T, &mut Ctx)) {
        // keep this body small due to monomorphization

        let mut joined = Scope::default();
        for item in it {
            let ctx = &mut self.enter();
            f(item, ctx);
            ctx.join_aux(&mut joined);
        }
        for (k, v) in joined {
            self.insert(k, v);
        }
    }
    fn join_aux(&mut self, scope: &mut Scope) {
        for map in self.scopes[self.index..].iter() {
            for (k, v) in map {
                scope.entry(k.clone())
                // TODO: change when Ty is copy()
                .and_modify(|v2| *v2 = subtype::join(&v, &v2)  /* use join here*/)
                .or_insert((*v).clone());
            }
        }
    }
    pub fn add_diagnostic<F>(&mut self, f: F)
    where
        F: FnOnce() -> Diagnostic,
    {
        if self.diagnose {
            self.diagnostics.push(f());
        }
    }
    pub fn looking_for(&self, expr_with_pos: &WithPos<Expr>) -> bool {
        self.look_for.as_ref() == Some(expr_with_pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Ty;

    fn x() -> String {
        "x".to_string()
    }

    #[test]
    fn test_scopes() {
        let ctx = &mut RootCtx::new(true, None);
        let ctx = &mut ctx.enter();
        ctx.insert("a".to_string(), Ty::Num);
        ctx.join([1, 2, 3].into_iter(), |n, ctx| {
                ctx.insert("a".to_string(), Ty::Bool);
        });
        println!("{:?}", ctx)
    }
}
