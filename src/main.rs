use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use bin_pack_wasm::packing::{Container, EbAfit, Item};

fn main() {
    let path = PathBuf::from("./test.txt");
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let mut counter = 1;
    let mut lines = reader.lines();
    loop {
        if counter > 700 {
            break;
        }
        lines.next(); // Skipping the ID line
        let str = lines.next().unwrap().unwrap();
        let test_results: Vec<_> = str.split(' ').collect();
        let str = lines.next().unwrap().unwrap();
        let container_dims: Vec<_> = str.split(' ').collect();
        let item_type_count: usize = lines.next().unwrap().unwrap().parse().unwrap();

        let mut items_to_pack = Vec::with_capacity(item_type_count);

        for _ in 0..item_type_count {
            let line_string = lines.next().unwrap().unwrap();
            let item_array: Vec<_> = line_string.split(' ').collect();
            let item = Item::new(
                item_array[1].parse::<f64>().unwrap(),
                item_array[3].parse::<f64>().unwrap(),
                item_array[5].parse::<f64>().unwrap(),
                item_array[7].parse::<usize>().unwrap(),
            );
            items_to_pack.push(item);
        }

        let container = Container::new(
            container_dims[0].parse::<f64>().unwrap(),
            container_dims[1].parse::<f64>().unwrap(),
            container_dims[2].parse::<f64>().unwrap(),
        );

        let mut algo = EbAfit::from_input(&container, &items_to_pack);
        algo.pack();
        let mut total_packed_items = 0;
        // let mut total_unpacked_items = 0;
        for st in algo.item_packing_status.iter() {
            if *st.1 {
                total_packed_items += 1;
            } else {
                // total_unpacked_items += 1;
            }
        }
        // algo.item_packing_status.iter().reduce(|acc, n| {
        //     if *n.1 {
        //         acc + 1;
        //     } else {
        //         acc
        //     }
        // });

        // assert_eq!(
        //     result[0].algorithm_packing_results[0].packed_items.len() as f64
        //         + result[0].algorithm_packing_results[0].unpacked_items.len() as f64,
        //     test_results[1].parse::<f64>()?
        // );
        // println!(
        //     "{}:{}",
        //     total_packed_items + total_unpacked_items,
        //     test_results[1].parse::<usize>().unwrap()
        // );
        println!(
            "{}:{}",
            total_packed_items,
            test_results[2].parse::<usize>().unwrap()
        );
        // assert!(
        //     (result[0].algorithm_packing_results[0].percent_container_volume_packed
        //         == test_results[3].parse::<f64>()?)
        //         || (result[0].algorithm_packing_results[0].percent_container_volume_packed
        //             == 87.20
        //             && test_results[3].parse::<f64>()? == 87.21)
        // );
        // assert_eq!(
        //     result[0].algorithm_packing_results[0].percent_item_volume_packed,
        //     test_results[4].parse::<f64>()?
        // );

        counter += 1;
    }
}
