use crate::data::*;

pub fn is_subtype(t1: &Ty, t2: &Ty) -> bool {
    t1 == t2 || match (t1, t2) {
        (Ty::Union(ty1, ty2), _) =>
            is_subtype(ty1, ty2) &&
            is_subtype(ty2, ty2),
        (_, Ty::Union(ty1, ty2)) =>
            is_subtype(ty1, ty2) ||
            is_subtype(ty2, ty2),
        _ => false,
    }
}

pub fn join(t1: &Ty, t2: &Ty) -> Ty {
    if is_subtype(t1, t2) {
        t1.to_owned()
    } else {
        Ty::Union(Box::new(t1.to_owned()), Box::new(t2.to_owned()))
    }
}
