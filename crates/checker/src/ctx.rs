use crate::data::*;
use std::collections::HashMap;

type Scope = HashMap<String, Ty>;

#[derive(Debug)]
pub struct Ctx<'a> {
    diagnostics: &'a mut Vec<Diagnostic>,
    scopes: &'a mut Vec<Scope>,
    index: usize,
    diagnose: bool,
    look_for: &'a Option<WithPos<Expr>>,
}

impl<'a> Ctx<'a> {
    pub fn new(
        diagnostics: &'a mut Vec<Diagnostic>,
        scopes: &'a mut Vec<Scope>,
        diagnose: bool,
        look_for: &'a Option<WithPos<Expr>>,
    ) -> Self {
        let index = scopes.len().max(1) - 1;
        Self {
            diagnostics,
            scopes,
            index,
            diagnose,
            look_for,
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
    pub fn get(&self, key: &str) -> Option<&Ty> {
        self.scopes.iter().rev().find_map(|s| s.get(key))
    }
    pub fn insert(&mut self, key: String, val: Ty) {
        let g = &mut self.scopes[self.index];
        g.insert(key, val);
    }
    pub fn enter<T>(&mut self, cb: &mut impl FnMut(&mut Ctx) -> T) -> T {
        self.scopes.push(Default::default());
        let index = self.scopes.len() - 1;
        let mut ctx = Ctx {
            diagnostics: self.diagnostics,
            scopes: self.scopes,
            index,
            diagnose: self.diagnose,
            look_for: self.look_for,
        };
        let res = cb(&mut ctx);
        self.scopes.remove(index);
        res
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
        let mut diagnostics = Vec::new();
        let mut scopes = Vec::new();
        let mut ctx = Ctx::new(&mut diagnostics, &mut scopes, true, &None);
        ctx.enter(&mut |ctx| {
            ctx.insert(x(), Ty::Bool);
            assert!(ctx.get("x").unwrap() == &Ty::Bool);
            ctx.enter(&mut |ctx| {
                assert!(ctx.get("x").unwrap() == &Ty::Bool);
                ctx.insert(x(), Ty::Num);
                assert!(ctx.get("x").unwrap() == &Ty::Num);
            });
            assert!(ctx.get("x").unwrap() == &Ty::Bool);
        });
    }
}
