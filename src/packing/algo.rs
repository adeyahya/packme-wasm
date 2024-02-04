use std::{collections::HashMap, default};

use super::{item::ItemDimension, Container, Item};

#[derive(Default)]
struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

struct Layer {
    pub layer_dim: f64,
    pub layer_eval: f64,
}

#[derive(Default, Clone)]
struct Scrappad {
    cumx: f64,
    cumz: f64,
    pre: Option<Box<Scrappad>>,
    pos: Option<Box<Scrappad>>,
}

pub struct EbAfit<'a> {
    pub container: &'a Container,
    item_list: Vec<Item>,
    item_packing_status: HashMap<usize, bool>,
    layer_list: Vec<Layer>,
    orientation_variant: OrientationVariant<'a>,
    orientation: Vector3,
    pub temp: f64,
    // sum of volume of all item / box
    pub total_box_vol: f64,
    // current box index that being evaluated
    bn: usize,
    same: bool,
    iteration_count: usize,
    packed_vol: f64,
    packedy: f64,
    is_packing: bool,
    layer_tickness: f64,
    remainpy: f64,
    remainpz: f64,
    packed_num_box: usize,
    is_quit: bool,
    scrappad: Scrappad,
    scrapfirst: Option<Box<Scrappad>>,
    scrapmemb: Option<Box<Scrappad>>,
    smallestz: Option<Box<Scrappad>>,
    trash: Option<Box<Scrappad>>,

    bfy: f64,
    bfx: f64,
    bfz: f64,
    boxx: f64,
    boxy: f64,
    boxz: f64,
    boxi: usize,
    bbfy: f64,
    bbfx: f64,
    bbfz: f64,
    bboxx: f64,
    bboxy: f64,
    bboxz: f64,
    bboxi: usize,
}

// public trait
impl<'a> EbAfit<'a> {
    pub fn from_input(container: &'a Container, item_list: &'a Vec<Item>) -> Self {
        let mut orientation_variant = OrientationVariant::from_container(container);
        let orientation = orientation_variant.next().unwrap();

        // flatten item list and filter out if the quantity are 0
        let mut computed_item_list = Vec::new();
        for item in item_list.iter() {
            if item.quantity == 0 {
                continue;
            };

            for _ in 0..item.quantity {
                let mut item = item.clone();
                computed_item_list.push(item);
            }
        }

        let total_box_vol = computed_item_list
            .iter()
            .map(|n| n.get_volume())
            .reduce(|acc, n| acc + n)
            .unwrap_or(0.0);

        Self {
            container,
            item_list: computed_item_list,
            item_packing_status: HashMap::new(),
            layer_list: Vec::new(),
            orientation_variant,
            orientation,
            total_box_vol,
            same: false,
            temp: 0.0,
            bn: 0,
            iteration_count: 0,
            packed_vol: 0.0,
            packedy: 0.0,
            is_packing: false,
            layer_tickness: 0.0,
            remainpy: 0.0,
            remainpz: 0.0,
            packed_num_box: 0,
            is_quit: false,
            scrappad: Scrappad::default(),
            scrapfirst: Some(Box::new(Scrappad::default())),
            scrapmemb: None,
            smallestz: None,
            trash: None,
            bfy: 0.0,
            bfx: 0.0,
            bfz: 0.0,
            boxx: 0.0,
            boxy: 0.0,
            boxz: 0.0,
            boxi: 0,
            bbfy: 0.0,
            bbfx: 0.0,
            bbfz: 0.0,
            bboxx: 0.0,
            bboxy: 0.0,
            bboxz: 0.0,
            bboxi: 0,
        }
    }

    pub fn pack(&mut self) {
        while self.next().is_some() {}
    }
}

// private trait
impl<'a> EbAfit<'a> {
    fn get_current_item(&self) -> Option<Item> {
        if let Some(item) = self.item_list.get(self.bn) {
            Some(item.clone())
        } else {
            None
        }
    }

    fn compute_candit_layer(&mut self, item: &Item) {
        let mut exdim: f64;
        let mut dimdif: f64;
        let mut dimen2: f64;
        let mut dimen3: f64;
        let mut layer_eval = 0.0;
        let py = self.orientation.y;
        let px = self.orientation.x;
        let pz = self.orientation.z;

        for y in 1..=3 {
            match y {
                1 => {
                    exdim = item.dim.0;
                    dimen2 = item.dim.1;
                    dimen3 = item.dim.2;
                }
                2 => {
                    exdim = item.dim.1;
                    dimen2 = item.dim.0;
                    dimen3 = item.dim.2;
                }
                3 => {
                    exdim = item.dim.2;
                    dimen2 = item.dim.0;
                    dimen3 = item.dim.1;
                }
                _ => unreachable!(),
            }

            if exdim > py || (dimen2 > px || dimen3 > pz) && (dimen3 > px || dimen2 > pz) {
                continue;
            }

            self.same = false;

            for k in 1..=self.layer_list.len() {
                if let Some(layer) = self.layer_list.get(k) {
                    if exdim == layer.layer_dim {
                        self.same = true;
                        continue;
                    }
                }
            }

            if self.same {
                continue;
            }

            for z in 1..=self.item_list.len() {
                if let Some(compared_item) = self.item_list.get(z) {
                    if item != compared_item {
                        dimdif = (exdim - compared_item.dim.0).abs().min(
                            (exdim - compared_item.dim.1)
                                .abs()
                                .min((exdim - compared_item.dim.2).abs()),
                        );
                        layer_eval += dimdif;
                    }
                }
            }

            self.layer_list.push(Layer {
                layer_eval,
                layer_dim: exdim,
            });
        }
    }

    //********************************************************
    // FINDS THE FIRST TO BE PACKED GAP IN THE LAYER EDGE
    //********************************************************
    fn find_smallest_z(&mut self) {
        if let Some(scrapfirst) = self.scrapfirst.as_mut() {
            self.scrapmemb = Some(scrapfirst.clone());
            self.smallestz = Some(scrapfirst.clone());

            let smallestz = self.smallestz.as_mut().unwrap();
            let scrapmemb = self.scrapmemb.as_mut().unwrap();
            while let Some(pos) = scrapmemb.pos.as_deref() {
                if pos.cumz < smallestz.cumz {
                    *smallestz = Box::new(pos.clone());
                }
                *scrapmemb = Box::new(pos.clone());
            }
        }
    }

    //**********************************************************************
    // PACKS THE BOXES FOUND AND ARRANGES ALL VARIABLES AND
    // RECORDS PROPERLY
    //**********************************************************************
    fn pack_layer(&mut self) -> bool {
        let mut lenx = 0.0;
        let mut lenz = 0.0;
        let mut lpz = 0.0;
        if self.layer_tickness == 0.0 {
            self.is_packing = false;
            return false;
        };

        if let Some(scrapfirst) = self.scrapfirst.as_mut() {
            scrapfirst.cumx = self.orientation.x;
            scrapfirst.cumz = 0.0;
        }

        while self.is_quit == false {
            self.find_smallest_z();
            if let Some(smallestz) = self.smallestz.as_deref() {
                match (smallestz.pre.as_deref(), smallestz.pos.as_deref()) {
                    //*** SITUATION-1: NO BOXES ON THE RIGHT AND LEFT SIDES ***
                    (None, None) => {
                        lenx = smallestz.cumx;
                        lpz = self.remainpz - smallestz.cumz;
                        self.find_box(lenx, self.layer_tickness, self.remainpy, lpz, lpz);
                    }
                    //*** SITUATION-2: NO BOXES ON THE LEFT SIDE ***
                    (None, _) => {}
                    //*** SITUATION-3: NO BOXES ON THE RIGHT SIDE ***
                    (_, None) => {}
                    (Some(pre), Some(pos)) => {
                        //*** SITUATION-4: THERE ARE BOXES ON BOTH OF THE SIDES ***

                        //*** SUBSITUATION-4A: SIDES ARE EQUAL TO EACH OTHER ***
                        if pre.cumz == pos.cumz {}
                    }
                }
            }
        }

        false
    }

    //**********************************************************************
    // ANALYZES EACH UNPACKED BOX TO FIND THE BEST FITTING ONE TO
    // THE EMPTY SPACE GIVEN
    //**********************************************************************
    fn analyze_box(
        &mut self,
        x: usize,
        hmx: f64,
        hy: f64,
        hmy: f64,
        hz: f64,
        hmz: f64,
        dim1: f64,
        dim2: f64,
        dim3: f64,
    ) {
        if dim1 <= hmx && dim2 <= hmy && dim3 <= hmz {
            if dim2 <= hy {
                if hy - dim2 < self.bfy {
                    self.boxx = dim1;
                    self.boxy = dim2;
                    self.boxz = dim3;
                    self.bfx = hmx - dim1;
                    self.bfy = hy - dim2;
                    self.bfz = (hz - dim3).abs();
                    self.boxi = x;
                } else if hy - dim2 == self.bfy && hmx - dim1 < self.bfx {
                    self.boxx = dim1;
                    self.boxy = dim2;
                    self.boxz = dim3;
                    self.bfx = hmx - dim1;
                    self.bfy = hy - dim2;
                    self.bfz = (hz - dim3).abs();
                    self.boxi = x;
                } else if hy - dim2 == self.bfy
                    && hmx - dim1 == self.bfx
                    && (hz - dim3).abs() < self.bfz
                {
                    self.boxx = dim1;
                    self.boxy = dim2;
                    self.boxz = dim3;
                    self.bfx = hmx - dim1;
                    self.bfy = hy - dim2;
                    self.bfz = (hz - dim3).abs();
                    self.boxi = x;
                }
            } else {
                if dim2 - hy < self.bbfy {
                    self.bboxx = dim1;
                    self.bboxy = dim2;
                    self.bboxz = dim3;
                    self.bbfx = hmx - dim1;
                    self.bbfy = dim2 - hy;
                    self.bbfz = (hz - dim3).abs();
                    self.bboxi = x;
                } else if dim2 - hy == self.bbfy && hmx - dim1 < self.bbfx {
                    self.bboxx = dim1;
                    self.bboxy = dim2;
                    self.bboxz = dim3;
                    self.bbfx = hmx - dim1;
                    self.bbfy = dim2 - hy;
                    self.bbfz = (hz - dim3).abs();
                    self.bboxi = x;
                } else if dim2 - hy == self.bbfy
                    && hmx - dim1 == self.bbfx
                    && (hz - dim3).abs() < self.bbfz
                {
                    self.bboxx = dim1;
                    self.bboxy = dim2;
                    self.bboxz = dim3;
                    self.bbfx = hmx - dim1;
                    self.bbfy = dim2 - hy;
                    self.bbfz = (hz - dim3).abs();
                    self.bboxi = x;
                }
            }
        }
    }

    //**********************************************************************
    // FINDS THE MOST PROPER BOXES BY LOOKING AT ALL SIX POSSIBLE
    // ORIENTATIONS, EMPTY SPACE GIVEN, ADJACENT BOXES, AND PALLET LIMITS
    //**********************************************************************
    fn find_box(&mut self, hmx: f64, hy: f64, hmy: f64, hz: f64, hmz: f64) -> () {
        self.bfx = f64::MAX;
        self.bfy = f64::MAX;
        self.bfz = f64::MAX;
        self.bbfx = f64::MAX;
        self.bbfy = f64::MAX;
        self.bbfz = f64::MAX;
        self.boxi = 0;
        self.bboxi = 0;
        let mut y = 0;
        let mut x = 0;
        let item_list = self.item_list.clone();
        let packing_status = self.item_packing_status.clone();
        while y < item_list.len() {
            if let Some(item_y) = item_list.get(y) {
                while x < x + item_y.quantity - 1 {
                    if let Some(is_packed) = packing_status.get(&x) {
                        if !*is_packed {
                            break;
                        }
                    }
                    x += 1;
                }
                if let Some(is_packed) = packing_status.get(&x) {
                    if *is_packed {
                        continue;
                    }
                }
                if x > item_list.len() {
                    return ();
                }
                let dim_x = match item_list.get(x) {
                    Some(item) => item.dim.clone(),
                    None => ItemDimension::default(),
                };
                self.analyze_box(x, hmx, hy, hmy, hz, hmz, dim_x.0, dim_x.1, dim_x.2);
                if dim_x.0 == dim_x.2 && dim_x.2 == dim_x.1 {
                    continue;
                }
                self.analyze_box(x, hmx, hy, hmy, hz, hmz, dim_x.0, dim_x.2, dim_x.1);
                self.analyze_box(x, hmx, hy, hmy, hz, hmz, dim_x.1, dim_x.0, dim_x.2);
                self.analyze_box(x, hmx, hy, hmy, hz, hmz, dim_x.1, dim_x.2, dim_x.0);
                self.analyze_box(x, hmx, hy, hmy, hz, hmz, dim_x.0, dim_x.2, dim_x.1);
                self.analyze_box(x, hmx, hy, hmy, hz, hmz, dim_x.2, dim_x.0, dim_x.1);
                self.analyze_box(x, hmx, hy, hmy, hz, hmz, dim_x.2, dim_x.1, dim_x.0);

                y += item_y.quantity;
            }
        }
        ()
    }
}

// implement iterator for the computation loop
// this is important for visualization
impl<'a> Iterator for EbAfit<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.get_current_item();
        if item.is_some() {
            self.compute_candit_layer(&item.unwrap());
            self.bn += 1;
            Some(())
        } else {
            // sets the evaluation of the first layer in the list to -1
            if let Some(layer) = self.layer_list.get_mut(0) {
                layer.layer_eval = -1.0;
            }
            // sort layer list
            self.layer_list.sort_by(|a, b| {
                if a.layer_eval < b.layer_eval {
                    std::cmp::Ordering::Less
                } else if a.layer_eval > b.layer_eval {
                    std::cmp::Ordering::Greater
                } else {
                    a.layer_dim
                        .partial_cmp(&b.layer_dim)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
            });

            self.item_packing_status = HashMap::new();
            let mut orientation_peekable = self.orientation_variant.clone().peekable();
            let orientation = orientation_peekable.peek().unwrap();
            for layer in self.layer_list.iter() {
                self.iteration_count += 1;
                self.packed_vol = 0.0;
                self.packedy = 0.0;
                self.is_packing = true;
                self.layer_tickness = layer.layer_dim;
                self.remainpy = orientation.y;
                self.remainpz = orientation.z;
                self.packed_num_box = 0 as usize;
            }

            // eof loop for current orientation
            // proceed to next orientation if available
            if let Some(orientation) = self.orientation_variant.next() {
                self.bn = 0;
                self.orientation = orientation;
                self.next()
            } else {
                None
            }
        }
    }
}

// iterator that represent 6 possible different orientation
// of the container
#[derive(Clone)]
struct OrientationVariant<'a> {
    current_variant: usize,
    container: &'a Container,
}

impl<'a> OrientationVariant<'a> {
    pub fn from_container(container: &'a Container) -> Self {
        Self {
            current_variant: 0,
            container,
        }
    }
}

impl<'a> Iterator for OrientationVariant<'a> {
    type Item = Vector3;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_variant {
            0 => Some(Vector3 {
                x: self.container.width,
                y: self.container.height,
                z: self.container.length,
            }),
            1 => Some(Vector3 {
                x: self.container.length,
                y: self.container.height,
                z: self.container.width,
            }),
            2 => Some(Vector3 {
                x: self.container.length,
                y: self.container.width,
                z: self.container.height,
            }),
            3 => Some(Vector3 {
                x: self.container.height,
                y: self.container.width,
                z: self.container.length,
            }),
            4 => Some(Vector3 {
                x: self.container.width,
                y: self.container.length,
                z: self.container.height,
            }),
            5 => Some(Vector3 {
                x: self.container.height,
                y: self.container.length,
                z: self.container.width,
            }),
            _ => None,
        }
    }
}
