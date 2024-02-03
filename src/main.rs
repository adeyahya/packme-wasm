mod packing;
use packing::*;
fn main() {
    let items = vec![
        BinItem {
            name: "Item 1".to_owned(),
            width: 3.0,
            height: 3.0,
            depth: 3.0,
        },
        BinItem {
            name: "Item 2".to_owned(),
            width: 5.0,
            height: 5.0,
            depth: 5.0,
        },
        BinItem {
            name: "Item 3".to_owned(),
            width: 7.0,
            height: 7.0,
            depth: 7.0,
        },
    ];
    let bin1 = Bin {
        name: "Bin 1".to_owned(),
        width: 2.0,
        height: 2.0,
        depth: 2.0,
        items: Vec::new(),
    };
    let bin2 = Bin {
        name: "Bin 2".to_owned(),
        width: 8.0,
        height: 8.0,
        depth: 8.0,
        items: Vec::new(),
    };
    let bins = vec![bin1, bin2];
    let mut bin_packing = BinPacking { bins };
    bin_packing.pack_items(items)
}
