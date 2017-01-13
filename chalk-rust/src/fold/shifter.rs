use errors::*;
use ir::*;
use super::{Fold, Folder};

pub struct Shifter {
    adjustment: usize
}

impl Shifter {
    pub fn new(adjustment: usize) -> Shifter {
        Shifter { adjustment }
    }

    pub fn up_shift<T: Fold>(adjustment: usize, value: &T) -> T::Result {
        value.fold_with(&mut Shifter::new(adjustment), 0).unwrap()
    }
}

impl Folder for Shifter {
    fn fold_free_var(&mut self, depth: usize, binders: usize) -> Result<Ty> {
        Ok(Ty::Var(depth + self.adjustment + binders))
    }

    fn fold_free_lifetime_var(&mut self, depth: usize, binders: usize) -> Result<Lifetime> {
        Ok(Lifetime::Var(depth + self.adjustment + binders))
    }
}



