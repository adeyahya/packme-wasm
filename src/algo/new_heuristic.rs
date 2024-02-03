use crate::packing::{Bin, BinItem};

pub struct NewHeuristic {
    is_full: bool,
    bins: Vec<Bin>,
}

pub fn compute(bins: &mut Vec<Bin>, items: &Vec<BinItem>) {}
