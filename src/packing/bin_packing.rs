use super::{Bin, BinItem};

#[derive(Default)]
pub struct BinPacking {
    pub bins: Vec<Bin>,
}

impl BinPacking {
    pub fn pack_items(&mut self, items: Vec<BinItem>) {
        let mut item_iter = items.iter().peekable();
        while let Some(item) = item_iter.peek() {
            let mut bin_iter = self.bins.iter_mut().peekable();
            while let Some(bin) = bin_iter.peek_mut() {
                if bin.add_item(item.to_owned().clone()) {
                    break;
                } else {
                    bin_iter.next();
                }
            }
            item_iter.next();
        }
    }
}
