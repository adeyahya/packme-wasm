use super::BinItem;

#[derive(Default, Clone)]
pub struct Bin {
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub items: Vec<BinItem>,
}

impl Bin {
    pub fn is_can_fit(&self, item: &BinItem) -> bool {
        item.width <= self.width && item.height <= self.height && item.depth <= item.depth
    }

    pub fn add_item(&mut self, item: BinItem) -> bool {
        if self.is_can_fit(&item) {
            self.items.push(item);
            return true;
        } else {
            return false;
        }
    }
}
