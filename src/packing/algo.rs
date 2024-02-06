use std::collections::HashMap;

use super::{item::ItemDimension, Container, Item};

#[derive(Default)]
struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone)]
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
    pub best_orientation_variant: usize,
    pub best_vol: f64,
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
    pre_layer: f64,
    is_packing_best: bool,
    is_hundred_percent_packed: bool,
    is_quit: bool,
    scrapfirst: Option<Box<Scrappad>>,
    scrapmemb: Option<Box<Scrappad>>,
    smallestz: Option<Box<Scrappad>>,
    trash: Option<Box<Scrappad>>,

    px: f64,
    bfy: f64,
    bfx: f64,
    bfz: f64,
    boxx: f64,
    boxy: f64,
    boxz: f64,
    boxi: Option<usize>,
    cboxi: Option<usize>,
    bboxi: Option<usize>,
    cboxx: f64,
    cboxy: f64,
    cboxz: f64,
    bbfy: f64,
    bbfx: f64,
    bbfz: f64,
    bboxx: f64,
    bboxy: f64,
    bboxz: f64,
    lilz: f64,
    evened: bool,
    layer_done: bool,
    layer_in_layer: Option<f64>,
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
                let item = item.clone();
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
            best_orientation_variant: 0,
            orientation,
            total_box_vol,
            same: false,
            temp: 0.0,
            bn: 0,
            iteration_count: 0,
            packed_vol: 0.0,
            packedy: 0.0,
            is_packing: false,
            is_hundred_percent_packed: false,
            is_packing_best: false,
            layer_tickness: 0.0,
            pre_layer: 0.0,
            remainpy: 0.0,
            remainpz: 0.0,
            packed_num_box: 0,
            is_quit: false,
            scrapfirst: Some(Box::new(Scrappad::default())),
            scrapmemb: None,
            smallestz: None,
            trash: None,
            px: 0.0,
            bfy: 0.0,
            bfx: 0.0,
            bfz: 0.0,
            boxx: 0.0,
            boxy: 0.0,
            boxz: 0.0,
            boxi: None,
            cboxi: None,
            bboxi: None,
            cboxx: 0.0,
            cboxy: 0.0,
            cboxz: 0.0,
            bbfy: 0.0,
            bbfx: 0.0,
            bbfz: 0.0,
            bboxx: 0.0,
            bboxy: 0.0,
            bboxz: 0.0,
            lilz: 0.0,
            best_vol: 0.0,
            evened: false,
            layer_done: false,
            layer_in_layer: None,
        }
    }

    pub fn pack(&mut self) {
        while !self.is_hundred_percent_packed && self.next().is_some() {}
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
    fn pack_layer(&mut self) -> () {
        let mut lenx = 0.0;
        let mut lenz = 0.0;
        let mut lpz = 0.0;
        if self.layer_tickness == 0.0 {
            self.is_packing = false;
            return ();
        };

        if let Some(scrapfirst) = self.scrapfirst.as_mut() {
            scrapfirst.cumx = self.orientation.x;
            scrapfirst.cumz = 0.0;
        }

        while self.is_quit == false {
            self.find_smallest_z();
            if let Some(smz) = self.smallestz.clone().as_deref_mut() {
                match (smz.pre.as_deref_mut(), smz.pos.as_deref_mut()) {
                    //*** SITUATION-1: NO BOXES ON THE RIGHT AND LEFT SIDES ***
                    (None, None) => {
                        self.smallestz.as_mut().map(|n| {
                            lenx = n.cumx;
                            lpz = self.remainpz - n.cumz;
                            n
                        });
                        self.find_box(lenx, self.layer_tickness, self.remainpy, lpz, lpz);
                        self.check_found();
                        if self.layer_done {
                            break;
                        };
                        if self.evened {
                            continue;
                        }

                        if let Some(item_cboxi) = self.item_list.get_mut(self.cboxi.unwrap_or(0)) {
                            item_cboxi.coord.x = 0.0;
                            item_cboxi.coord.y = self.packedy;
                            item_cboxi.coord.z =
                                self.smallestz.as_ref().map(|n| n.cumz).unwrap_or(0.0);
                        }

                        if self.smallestz.is_some() {
                            let smallestz = self.smallestz.as_deref_mut().unwrap();
                            if self.cboxx == smallestz.cumx {
                                smallestz.cumz = smallestz.cumz + self.cboxz;
                            } else {
                                smallestz.pos = Some(Box::new(Scrappad::default()));
                                let mut s_pos = smallestz.pos.clone().unwrap();
                                s_pos.pos = None;
                                s_pos.pre = Some(Box::new(smallestz.clone()));
                                s_pos.cumx = smallestz.cumx;
                                s_pos.cumz = smallestz.cumz;
                                smallestz.cumx = self.cboxx;
                                smallestz.cumz = smallestz.cumz + self.cboxz;
                            }
                        }
                        self.volume_check();
                    }
                    //*** SITUATION-2: NO BOXES ON THE LEFT SIDE ***
                    (None, _) => {
                        self.smallestz.as_ref().map(|n| {
                            lenx = n.cumx;
                            n.pos.as_ref().map(|npos| {
                                lenz = npos.cumz - n.cumz;
                            });
                            lpz = self.remainpz - n.cumz;
                        });

                        self.find_box(lenx, self.layer_tickness, self.remainpy, lenz, lpz);
                        self.check_found();
                        if self.layer_done {
                            break;
                        }
                        if self.evened {
                            continue;
                        }

                        let cboxi = self.cboxi.unwrap_or(0);
                        if let Some(item_cboxi) = self.item_list.get_mut(cboxi) {
                            item_cboxi.coord.y = self.packedy;
                            item_cboxi.coord.z =
                                self.smallestz.as_ref().map(|n| n.cumz).unwrap_or(0.0);

                            let (cumx, cumz) = self
                                .smallestz
                                .as_ref()
                                .map(|n| (n.cumx, n.cumz))
                                .unwrap_or((0.0, 0.0));
                            if self.cboxx == cumx {
                                item_cboxi.coord.x = 0.0;
                                let (pcumx, pcumz) = self
                                    .smallestz
                                    .as_ref()
                                    .map(|n| {
                                        if let Some(np) = &n.pos {
                                            return (np.cumx, np.cumz);
                                        } else {
                                            return (0.0, 0.0);
                                        }
                                    })
                                    .unwrap_or((0.0, 0.0));
                                if cumz + self.cboxz == pcumz {
                                    self.smallestz.as_mut().map(|n| {
                                        n.cumz = pcumz;
                                        n.cumx = pcumx;
                                    });
                                    self.trash = self.smallestz.clone().unwrap_or_default().pos;
                                    let spp = self
                                        .smallestz
                                        .clone()
                                        .unwrap_or_default()
                                        .pos
                                        .unwrap_or_default()
                                        .pos;
                                    self.smallestz.as_mut().map(|n| n.pos = spp);

                                    let smallest_clone = self.smallestz.clone();
                                    // smallestz->pos->pre = smallestz
                                    self.smallestz.as_mut().map(|n| {
                                        n.pos.as_mut().map(|np| {
                                            np.pre = smallest_clone;
                                        })
                                    });
                                } else {
                                    let cboxz = self.cboxz;
                                    self.smallestz
                                        .as_deref_mut()
                                        .map(|n| n.cumz = n.cumz + cboxz);
                                }
                            } else {
                                let smallestz = self.smallestz.clone().unwrap_or_default();
                                self.item_list.get_mut(self.cboxi.unwrap_or(0)).map(|item| {
                                    item.coord.x = smallestz.cumx - self.cboxx;
                                });
                                if smallestz.cumz + self.cboxz
                                    == smallestz.pos.unwrap_or_default().cumz
                                {
                                    self.smallestz
                                        .as_deref_mut()
                                        .map(|n| n.cumx = n.cumx - self.cboxx);
                                } else {
                                    // smallestZ.Post.Pre = new ScrapPad();
                                    self.smallestz.as_mut().map(|n| {
                                        n.pos.as_deref_mut().map(|npos| {
                                            npos.pre = Some(Box::new(Scrappad::default()));
                                        });
                                    });
                                    let smallestz = self.smallestz.clone();
                                    let npos_clone = self.smallestz.clone().unwrap_or_default().pos;
                                    self.smallestz.as_mut().map(|n| {
                                        n.pos.as_deref_mut().map(|npos| {
                                            npos.pre.as_deref_mut().map(|npos_pre| {
                                                // smallestZ.Post.Pre.Post = smallestZ.Post;
                                                npos_pre.pos = npos_clone;
                                                // smallestZ.Post.Pre.Pre = smallestZ;
                                                npos_pre.pre = smallestz;
                                            });
                                        });
                                    });
                                    // smallestZ.Post = smallestZ.Post.Pre;
                                    let npos_pre_clone = self
                                        .smallestz
                                        .clone()
                                        .unwrap_or_default()
                                        .pos
                                        .unwrap_or_default()
                                        .pre;
                                    self.smallestz
                                        .as_deref_mut()
                                        .map(|n| n.pos = npos_pre_clone);

                                    // smallestZ.Post.CumX = smallestZ.CumX;
                                    self.smallestz.as_deref_mut().map(|n| {
                                        n.pos.as_deref_mut().map(|npos| {
                                            npos.cumx = n.cumx;
                                        });
                                        n.cumx = n.cumx - self.cboxx;
                                        // smallestZ.CumX = smallestZ.CumX - cboxx;
                                    });

                                    // smallestZ.Post.CumZ = smallestZ.CumZ + cboxz;
                                    self.smallestz.as_deref_mut().map(|n| {
                                        n.pos.as_deref_mut().map(|npos| {
                                            npos.cumz = n.cumz + self.cboxz;
                                        });
                                    });
                                }
                            }
                        }
                        self.volume_check();
                    }
                    //*** SITUATION-3: NO BOXES ON THE RIGHT SIDE ***
                    (Some(s_pre), None) => {
                        let smallestz = self.smallestz.clone().unwrap();
                        lenx = smallestz.cumx - s_pre.cumx;
                        lenz = s_pre.cumz - smallestz.cumz;
                        lpz = self.remainpz - smallestz.cumz;
                        self.find_box(lenx, self.layer_tickness, self.remainpy, lenz, lpz);
                        self.check_found();

                        if self.layer_done {
                            break;
                        };

                        if self.evened {
                            continue;
                        };

                        self.item_list.get_mut(self.cboxi.unwrap_or(0)).map(|n| {
                            n.coord.y = self.packedy;
                            n.coord.z = smallestz.cumz;
                            n.coord.x = s_pre.cumx;
                        });

                        if self.cboxx == smallestz.cumx - s_pre.cumx {
                            if smallestz.cumz + self.cboxz == s_pre.cumz {
                                self.smallestz.as_deref_mut().map(|n| {
                                    n.pre.as_deref_mut().map(|npre| {
                                        npre.cumx = n.cumx;
                                        npre.pos = None;
                                    });
                                });
                            } else {
                                self.smallestz.as_deref_mut().map(|n| {
                                    n.cumz = n.cumz + self.cboxz;
                                });
                            }
                        } else {
                            if smallestz.cumz + self.cboxz == s_pre.cumz {
                                self.smallestz.as_deref_mut().map(|n| {
                                    n.pre.as_deref_mut().map(|npre| {
                                        npre.cumx = npre.cumx + self.cboxx;
                                    });
                                });
                            } else {
                                self.smallestz.as_deref_mut().map(|n| {
                                    n.pre.as_deref_mut().map(|npre| {
                                        npre.pos = Some(Box::new(Scrappad::default()));
                                        npre.pos.as_deref_mut().map(|nprepos| {
                                            nprepos.pre = smallestz.pre.clone();
                                            nprepos.pos = Some(smallestz.clone());
                                        });
                                        *npre = *npre.pos.clone().unwrap();
                                        npre.pre.as_deref_mut().map(|nprepre| {
                                            npre.cumx = nprepre.cumx + self.cboxx;
                                            npre.cumz = n.cumz + self.cboxz;
                                        });
                                    });
                                });
                            }
                        }
                        self.volume_check();
                    }
                    //*** SITUATION-4: THERE ARE BOXES ON BOTH OF THE SIDES ***
                    (Some(mut pre), Some(mut pos)) => {
                        //*** SUBSITUATION-4A: SIDES ARE EQUAL TO EACH OTHER ***
                        if pre.cumz == pos.cumz {
                            self.smallestz.as_deref_mut().map(|n| {
                                n.pre.as_deref_mut().map(|npre| {
                                    lenx = n.cumx - npre.cumx;
                                    lenz = npre.cumz - n.cumz;
                                    lpz = self.remainpz - n.cumz;
                                });
                            });
                            self.find_box(lenx, self.layer_tickness, self.remainpy, lenz, lpz);
                            self.check_found();

                            if self.layer_done {
                                break;
                            };

                            if self.evened {
                                continue;
                            };

                            let smallestz = self.smallestz.clone().unwrap();
                            self.item_list
                                .get_mut(self.cboxi.clone().unwrap_or(0))
                                .map(|n| {
                                    n.coord.y = self.packedy;
                                    n.coord.z = smallestz.cumz;
                                });

                            if self.cboxx == smallestz.cumx - pre.cumx {
                                self.item_list
                                    .get_mut(self.cboxi.clone().unwrap_or(0))
                                    .map(|n| {
                                        n.coord.y = pre.cumx;
                                    });

                                if smallestz.cumz + self.cboxz == pos.cumz {
                                    pre.cumx = pos.cumx;

                                    if pos.pos.is_some() {
                                        pre.pos = pos.pos.clone();
                                        pos.pos.as_deref_mut().unwrap().pre =
                                            Some(Box::new(pre.clone()));
                                    } else {
                                        pre.pos = None;
                                    }
                                } else {
                                    self.smallestz.as_deref_mut().map(|n| {
                                        n.cumz = n.cumz + self.cboxz;
                                    });
                                }
                            } else if pre.cumx < self.px - smallestz.cumx {
                                if smallestz.cumx + self.cboxz == pre.cumz {
                                    self.smallestz.as_deref_mut().map(|n| {
                                        n.cumx = n.cumx - self.cboxx;
                                    });
                                    self.item_list.get_mut(self.cboxi.unwrap_or(0)).map(|n| {
                                        n.coord.x = self.smallestz.clone().unwrap().cumx;
                                    });
                                } else {
                                    self.item_list.get_mut(self.cboxi.unwrap_or(0)).map(|n| {
                                        n.coord.x = pre.cumx;
                                    });
                                    pre.pos = Some(Box::new(Scrappad::default()));

                                    pre.pos.as_deref_mut().unwrap().pre =
                                        Some(Box::new(pre.clone()));
                                    pre.pos.as_deref_mut().unwrap().pos = self.smallestz.clone();
                                    let mut prepos = pre.pos.clone();
                                    pre = prepos.as_deref_mut().unwrap();
                                    pre.cumx = pre.clone().pre.unwrap().cumx + self.cboxx;
                                    pre.cumz = self.smallestz.clone().unwrap().cumz + self.cboxz;
                                }
                            } else {
                                let smallestz = self.smallestz.clone().unwrap();
                                if smallestz.cumz + self.cboxz == pre.cumz {
                                    pre.cumx = pre.cumx + self.cboxx;
                                    self.item_list.get_mut(self.cboxi.unwrap_or(0)).map(|n| {
                                        n.coord.x = pre.cumx;
                                    });
                                } else {
                                    self.item_list.get_mut(self.cboxi.unwrap_or(0)).map(|n| {
                                        n.coord.x =
                                            self.smallestz.clone().unwrap().cumx - self.cboxx;
                                    });
                                    pos.pre = Some(Box::new(Scrappad::default()));

                                    pos.pre.as_deref_mut().unwrap().pos =
                                        Some(Box::new(pos.clone()));
                                    pos.pre.as_deref_mut().unwrap().pre = self.smallestz.clone();
                                    let mut pospre = pos.pre.clone();
                                    pos = pospre.as_deref_mut().unwrap();
                                    pos.cumx = self.smallestz.clone().unwrap().cumx;
                                    pos.cumx = self.smallestz.clone().unwrap().cumz + self.cboxz;
                                    self.smallestz.as_deref_mut().unwrap().cumx =
                                        self.smallestz.clone().unwrap().cumx - self.cboxx;
                                }
                            }
                            self.volume_check();
                        } else {
                            //*** SUBSITUATION-4B: SIDES ARE NOT EQUAL TO EACH OTHER ***
                            let smallestz = self.smallestz.clone().unwrap();
                            lenx = smallestz.cumx - pre.cumx;
                            lenz = pre.cumz - smallestz.cumz;
                            lpz = self.remainpz - smallestz.cumz;
                            self.find_box(lenx, self.layer_tickness, self.remainpy, lenz, lpz);
                            self.check_found();

                            if self.layer_done {
                                break;
                            }

                            if self.evened {
                                continue;
                            }

                            self.item_list.get_mut(self.cboxi.unwrap_or(0)).map(|n| {
                                n.coord.y = self.packedy;
                                n.coord.z = smallestz.cumz;
                                n.coord.x = pre.cumx;
                            });

                            if self.cboxx == smallestz.cumx - pre.cumx {
                                if smallestz.cumz + self.cboxz == pre.cumz {
                                    pre.cumx = smallestz.cumx;
                                    pre.pos = Some(Box::new(pos.clone()));
                                    pos.pre = Some(Box::new(pre.clone()));
                                } else {
                                    self.smallestz.as_deref_mut().unwrap().cumz =
                                        smallestz.cumz + self.cboxz;
                                }
                            } else {
                                if smallestz.cumz + self.cboxz == pre.cumz {
                                    pre.cumx = pre.cumx + self.cboxx;
                                } else if smallestz.cumz + self.cboxz == pos.cumz {
                                    self.item_list.get_mut(self.cboxi.unwrap_or(0)).map(|n| {
                                        n.coord.x = smallestz.cumx - self.cboxx;
                                    });
                                    self.smallestz.as_deref_mut().unwrap().cumx =
                                        smallestz.cumx - self.cboxx;
                                } else {
                                    pre.pos = Some(Box::new(Scrappad::default()));
                                    pre.pos.as_deref_mut().unwrap().pre =
                                        Some(Box::new(pre.clone()));
                                    pre.pos.as_deref_mut().unwrap().pos = self.smallestz.clone();
                                    let mut prepos = pre.pos.clone();
                                    pre = prepos.as_deref_mut().unwrap();
                                    pre.cumx = pre.pre.as_ref().unwrap().cumx + self.cboxx;
                                    pre.cumz = smallestz.cumz + self.cboxz;
                                }
                            }
                            self.volume_check();
                        }
                    }
                }
            }
        }
        ()
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
                    self.boxi = Some(x);
                } else if hy - dim2 == self.bfy && hmx - dim1 < self.bfx {
                    self.boxx = dim1;
                    self.boxy = dim2;
                    self.boxz = dim3;
                    self.bfx = hmx - dim1;
                    self.bfy = hy - dim2;
                    self.bfz = (hz - dim3).abs();
                    self.boxi = Some(x);
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
                    self.boxi = Some(x);
                }
            } else {
                if dim2 - hy < self.bbfy {
                    self.bboxx = dim1;
                    self.bboxy = dim2;
                    self.bboxz = dim3;
                    self.bbfx = hmx - dim1;
                    self.bbfy = dim2 - hy;
                    self.bbfz = (hz - dim3).abs();
                    self.bboxi = Some(x);
                } else if dim2 - hy == self.bbfy && hmx - dim1 < self.bbfx {
                    self.bboxx = dim1;
                    self.bboxy = dim2;
                    self.bboxz = dim3;
                    self.bbfx = hmx - dim1;
                    self.bbfy = dim2 - hy;
                    self.bbfz = (hz - dim3).abs();
                    self.bboxi = Some(x);
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
                    self.bboxi = Some(x);
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

    //************************************************************
    // AFTER FINDING EACH BOX, THE CANDIDATE BOXES AND THE
    // CONDITION OF THE LAYER ARE EXAMINED
    //************************************************************
    fn check_found(&mut self) {
        self.evened = false;
        if let Some(boxi) = self.boxi {
            self.cboxi = Some(boxi);
            self.cboxx = self.boxx;
            self.cboxy = self.boxy;
            self.cboxz = self.boxz;
        } else {
            if let Some(smallestz) = self.smallestz.as_deref_mut() {
                if self.bboxi.is_some()
                    && (self.layer_in_layer.is_some()
                        || smallestz.pre.is_none() && smallestz.pos.is_none())
                {
                    if self.layer_in_layer.is_none() {
                        self.pre_layer = self.layer_tickness;
                        self.lilz = smallestz.cumz;
                    }
                    self.cboxi = self.bboxi;
                    self.cboxx = self.bboxx;
                    self.cboxy = self.bboxy;
                    self.cboxz = self.bboxz;
                    let layer_in_layer = self.layer_in_layer.unwrap_or(0.0);
                    self.layer_in_layer = Some(layer_in_layer + self.bboxy - self.layer_tickness);
                    self.layer_tickness = self.bboxy;
                } else {
                    if smallestz.pre.is_none() && smallestz.pos.is_none() {
                        self.layer_done = true;
                    } else {
                        self.evened = true;
                        if smallestz.pre.is_none() {
                            self.trash = smallestz.pos.clone();
                            let s_pos = smallestz.pos.clone();
                            if let Some(s_pos) = s_pos.as_deref() {
                                smallestz.cumx = s_pos.cumx;
                                smallestz.cumz = s_pos.cumz;
                                smallestz.pos = s_pos.pos.clone();
                            }
                            let smallestz_clone = smallestz.clone();
                            if let Some(s_pos) = smallestz.pos.as_deref_mut() {
                                s_pos.pre = Some(Box::new(smallestz_clone));
                            }
                        } else if smallestz.pos.is_none() {
                            if let Some(s_pre) = smallestz.pre.as_deref_mut() {
                                s_pre.pos = None;
                                s_pre.cumx = smallestz.cumx;
                            }
                        } else {
                            if let (Some(s_pre), Some(s_pos)) =
                                (smallestz.pre.as_deref_mut(), smallestz.pos.as_deref_mut())
                            {
                                if s_pre.cumz == s_pos.cumz {
                                    s_pre.pos = s_pos.pos.clone();

                                    if s_pos.pos.is_some() {
                                        let s_pos_pos = s_pos.pos.as_deref_mut().unwrap();
                                        s_pos_pos.pre = Some(Box::new(s_pre.clone()));
                                    }

                                    s_pre.cumx = s_pos.cumx;
                                } else {
                                    let smallestz_clone = smallestz.clone();
                                    if let (Some(s_pre), Some(s_pos)) =
                                        (smallestz.pre.as_deref_mut(), smallestz.pos.as_deref_mut())
                                    {
                                        s_pre.pos = smallestz_clone.clone().pos.clone();
                                        s_pos.pre = smallestz_clone.clone().pre.clone();
                                        if s_pre.cumz < s_pos.cumz {
                                            s_pre.cumx = smallestz_clone.cumx;
                                        }
                                    }
                                }
                            }
                            self.smallestz = None;
                        }
                    }
                }
            }
        }
    }

    //************************************************************
    // After packing of each item, the 100% packing condition is checked.
    //************************************************************
    fn volume_check(&mut self) {
        self.item_packing_status
            .insert(self.cboxi.unwrap_or(0), true);
        self.item_list.get_mut(self.cboxi.unwrap_or(0)).map(|n| {
            n.pack_dim.0 = self.cboxx;
            n.pack_dim.1 = self.cboxy;
            n.pack_dim.2 = self.cboxz;
            self.packed_vol = self.packed_vol + n.get_volume();
            self.packed_num_box += 1;
        });

        if self.is_packing_best {
            println!("")
        } else if self.packed_vol == self.container.get_volume()
            || self.packed_vol == self.total_box_vol
        {
            self.is_packing = false;
            self.is_hundred_percent_packed = true;
        }
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
            self.is_packing_best = true;
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

            let mut orientation_peekable = self.orientation_variant.clone().peekable();
            let orientation = orientation_peekable.peek().unwrap();
            let layer_list = self.layer_list.clone();
            for layer in layer_list.iter() {
                self.iteration_count += 1;
                self.packed_vol = 0.0;
                self.packedy = 0.0;
                self.is_packing = true;
                self.layer_tickness = layer.layer_dim;
                self.remainpy = orientation.y;
                self.remainpz = orientation.z;
                self.packed_num_box = 0 as usize;
                self.item_packing_status = HashMap::new();

                while self.is_packing {
                    self.layer_in_layer = None;
                    self.layer_done = false;
                    self.pack_layer();
                    self.packedy = self.packedy + self.layer_tickness;
                    self.remainpy = orientation.y - self.packedy;

                    if self.layer_in_layer.is_some() {
                        let prepackedy = self.packedy;
                        let preremainpy = self.remainpy;
                        self.remainpy = self.layer_tickness - self.pre_layer;
                        self.packedy = self.packedy - self.layer_tickness + self.pre_layer;
                        self.remainpz = self.lilz;
                        self.layer_tickness = self.layer_in_layer.unwrap();
                        self.layer_done = false;
                        self.pack_layer();
                        self.packedy = prepackedy;
                        self.remainpy = preremainpy;
                        self.remainpz = orientation.z;
                    }
                }
            }

            if self.packed_vol > self.best_vol {
                self.best_vol = self.packed_vol;
                self.best_orientation_variant = self.orientation_variant.current_variant;
            }

            if self.is_hundred_percent_packed {
                return None;
            }

            if self.container.length == self.container.height
                && self.container.height == self.container.width
            {
                // this will skip to 5 on .next() call below
                self.orientation_variant.current_variant = 4;
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
    pub current_variant: usize,
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
