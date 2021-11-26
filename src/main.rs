mod grid;
mod krand;
mod image_output;
mod priority_queue;
mod constraint;

use krand::*;
use image_output::*;

use grid::*;
use priority_queue::*;
use constraint::*;


// todo flip, rot
#[derive(Copy, Clone)]
pub struct TileSpec {
    constraint: Constraint,
    px_colour: [(u8, u8, u8); 9],

    weight: f32,
}

type TileHandle = u32;
const HANDLE_FAILED_PLACEMENT: u32 = std::u32::MAX; // mmm C style, yucky?
const HANDLE_UNSET: u32 = std::u32::MAX - 1; // mmm C style, yucky?

pub struct TileSet {
    vec: Vec<TileSpec>,
}

impl TileSet {
    
    pub fn get_tilespec(&self, handle: TileHandle) -> TileSpec {
        if handle == HANDLE_FAILED_PLACEMENT {
            
            let b = (0, 0, 0);
            return TileSpec {
                constraint: Constraint {
                    r: 0, g: 0, b: 0, mask: 0,
                },
                px_colour: [b, b, b, b, b, b, b, b, b],
                weight: 0.0,
            };
        }
        if handle == HANDLE_UNSET {
            let m = (255, 0, 255);
            return TileSpec {
                constraint: Constraint {
                    r: 0, g: 0, b: 0, mask: 0,
                },
                px_colour: [m, m, m, m, m, m, m, m, m],
                weight: 0.0,
            };
        }

        self.vec[handle as usize]
    }

    pub fn n_tiles_satisfying_constraints(&self, c: Constraint) -> usize {
        self.vec.iter().filter(|ts| constraint_match(ts.constraint, c)).count()
    }

    pub fn get_tile_satisfying_constraints(&self, c: Constraint, seed: u32) -> Option<TileHandle> {
        let handle_weights: Vec<(TileHandle, f32)> = self.vec.iter()
            .map(|ts| (ts.constraint, ts.weight))
            .enumerate()
            .filter(|(handle, (constraint, weight))| constraint_match(*constraint, c))
            .map(|(handle, (constraint, weight))| (handle as TileHandle, weight))
            .collect();

        if handle_weights.len() == 0 {
            return None;
        }

        let weight_sum = handle_weights.iter().fold(0.0, |acc, (_, weight)| acc + weight);
        let choice = uniform_f32(seed) * weight_sum;
        let mut acc = 0.0;
        for (handle, weight) in handle_weights {
            acc += weight;
            if acc >= choice {
                return Some(handle);
            }
        }
        panic!("unreachable");
    }
}

pub fn generate_tiling(tileset: &TileSet, w: usize, h: usize, seed: u32) -> Vec<TileHandle> {
    let unconstrained = Constraint {
        r: 0,
        g: 0,
        b: 0,
        mask: 0xFFFFFFFFFFFFFFFF
    };

    // initialize a grid of constraints for working
    let mut constraint_grid = Grid::new(w, h, unconstrained);

    // tracks rerolls
    let mut generation_grid = Grid::new(w, h, 0u32);

    // initalize the output grid
    let mut output_grid = Grid::new(w, h, HANDLE_UNSET);

    // set up pq
    let mut pq = PriorityQueue::new();
    // put in all
    // update constraints, update pq listings as well

    for tile_i in 0..w {
        for tile_j in 0..h {
            pq.set(tileset.n_tiles_satisfying_constraints(constraint_grid.get(tile_i, tile_j)), (tile_i, tile_j));
        }
    }

    let mut small_rollbacks = 0;
    let mut med_rollbacks = 0;
    let mut big_rollbacks = 0;
    let mut place_failures = 0;


    while let Some(listing) = pq.remove_min() {
        
        let i = listing.0;
        let j = listing.1;


        let gen = generation_grid.get(i, j);
        if let Some(handle) = tileset.get_tile_satisfying_constraints(
                constraint_grid.get(i, j), 
                seed+(i as u32)+(0xF686CB1A * j as u32)+gen*0xCB497A23
            ) {

            output_grid.set(i, j, handle);
            for dir in [Dir::North ,Dir::East, Dir::South, Dir::West] {
                if let Some(neigh_constraint) = constraint_grid.neighbour_mut(i, j, dir) {
                    constraint_add(neigh_constraint, tileset.get_tilespec(handle).constraint, dir); // assuming dir and not opposite(dir)
                    let neigh_idx = idx_in_dir(i, j, dir);
                    if output_grid.get(neigh_idx.0, neigh_idx.1) == HANDLE_UNSET {
                        pq.set(tileset.n_tiles_satisfying_constraints(*neigh_constraint), neigh_idx);
                    }
                }
            }
        } else {
            let mut roll_back = |i: usize, j: usize, r: i32| {

                // zero internal cells
                for oi in -(r-1)..=(r-1) {
                    for oj in -(r-1)..=(r-1) {
                        if let Some(constraint) = constraint_grid.offset_mut(i, j, oi, oj) {
                            let ius = (i as i32 + oi) as usize;
                            let jus = (j as i32 + oj) as usize;
                            *constraint = unconstrained;
                            *generation_grid.get_mut(ius, jus) += 1;
                            output_grid.set(i, j, HANDLE_UNSET);
                            pq.set(tileset.n_tiles_satisfying_constraints(unconstrained), (ius, jus));
                        }
        
                    }
                }
        
                {
                    // western edge
                    let oi = -r;
                    for oj in -(r-1)..=(r-1) {
                        if let Some(constraint) = constraint_grid.offset_mut(i, j, oi, oj) {
                            let ius = (i as i32 + oi) as usize;
                            let jus = (j as i32 + oj) as usize;
                            constraint_add(constraint, unconstrained, Dir::West);
                            *generation_grid.get_mut(ius, jus) += 1;
                            output_grid.set(ius, jus, HANDLE_UNSET);
                            pq.set(tileset.n_tiles_satisfying_constraints(*constraint), (ius, jus));
                        }
                    }
                }
                {
                    // eastern edge
                    let oi = r;
                    for oj in -(r-1)..=(r-1) {
                        if let Some(constraint) = constraint_grid.offset_mut(i, j, oi, oj) {
                            let ius = (i as i32 + oi) as usize;
                            let jus = (j as i32 + oj) as usize;
                            constraint_add(constraint, unconstrained, Dir::East);
                            *generation_grid.get_mut(ius, jus) += 1;
                            output_grid.set(ius, jus, HANDLE_UNSET);
                            pq.set(tileset.n_tiles_satisfying_constraints(*constraint), (ius, jus));
                        }
                    }
                }
                {
                    // southern edge
                    let oj = r;
                    for oi in -(r-1)..=(r-1) {
                        if let Some(constraint) = constraint_grid.offset_mut(i, j, oi, oj) {
                            let ius = (i as i32 + oi) as usize;
                            let jus = (j as i32 + oj) as usize;
                            constraint_add(constraint, unconstrained, Dir::South);
                            *generation_grid.get_mut(ius, jus) += 1;
                            output_grid.set(ius, jus, HANDLE_UNSET);
                            pq.set(tileset.n_tiles_satisfying_constraints(*constraint), (ius, jus));
                        }
                    }
                }
                {
                    // northern edge
                    let oj = -r;
                    for oi in -(r-1)..=(r-1) {
                        if let Some(constraint) = constraint_grid.offset_mut(i, j, oi, oj) {
                            let ius = (i as i32 + oi) as usize;
                            let jus = (j as i32 + oj) as usize;
                            constraint_add(constraint, unconstrained, Dir::North);
                            *generation_grid.get_mut(ius, jus) += 1;
                            output_grid.set(ius, jus, HANDLE_UNSET);
                            pq.set(tileset.n_tiles_satisfying_constraints(*constraint), (ius, jus));
                        }
                    }
                }
            };
            if gen < 100 {
                small_rollbacks += 1;
                roll_back(i,j,1);
            } else if gen < 1000 {
                med_rollbacks += 1;
                roll_back(i,j,2);
            } else if gen < 0 {
                big_rollbacks += 1;
                roll_back(i,j,3);
            } else {
                place_failures += 1;
                output_grid.set(i, j, HANDLE_FAILED_PLACEMENT);
            }
        }
    }

    println!("difficulty -- small: {} med: {} big: {} fail: {}", small_rollbacks, med_rollbacks, big_rollbacks, place_failures);
    output_grid.elements
}



fn make_tileset(image: &ImageBuffer) -> TileSet {
    let n_tiles_x = image.w / 4;
    let n_tiles_y = image.w / 4;

    let mut t = TileSet {
        vec: Vec::new(),
    };

    for tile_i in 0..n_tiles_x {
        for tile_j in 0..n_tiles_y {
            let tile_px = |tx: usize, ty: usize| image.get_px(tile_i*4 + tx, tile_j*4 + ty);
            
            let comment = tile_px(3, 3) == (0xFF, 0x00, 0x00);
            let rotations = tile_px(3, 0) == (0x00, 0x00, 0xFF);

            if !comment {
                let weight_px = tile_px(0,3);
                let px_colour = [tile_px(0,0), tile_px(1,0), tile_px(2,0), 
                                    tile_px(0,1), tile_px(1,1), tile_px(2,1), 
                                    tile_px(0,2), tile_px(1,2), tile_px(2,2)];
                t.vec.push(TileSpec {
                    px_colour,
                    weight: weight_px.0 as f32 + weight_px.1 as f32 + weight_px.2 as f32 / (255.0*3.0),
                    constraint: constraint_from_px_colour(px_colour),
                });

                if rotations {
                    let rot90_px_colour = rot_tile(px_colour);
                    t.vec.push(TileSpec {
                        px_colour: rot90_px_colour,
                        weight: weight_px.0 as f32 + weight_px.1 as f32 + weight_px.2 as f32 / (255.0*3.0),
                        constraint: constraint_from_px_colour(rot90_px_colour),
                    });
                    let rot180_px_colour = rot_tile(rot90_px_colour);
                    t.vec.push(TileSpec {
                        px_colour: rot180_px_colour,
                        weight: weight_px.0 as f32 + weight_px.1 as f32 + weight_px.2 as f32 / (255.0*3.0),
                        constraint: constraint_from_px_colour(rot180_px_colour),
                    });
                    let rot270_px_colour = rot_tile(rot180_px_colour);
                    t.vec.push(TileSpec {
                        px_colour: rot270_px_colour,
                        weight: weight_px.0 as f32 + weight_px.1 as f32 + weight_px.2 as f32 / (255.0*3.0),
                        constraint: constraint_from_px_colour(rot270_px_colour),
                    });
                }


            }
        }
    }

    t
}

fn do_tiles(in_path: &str, out_path: &str, w: usize, h: usize, seed: u32) {
    println!("tiling {}...", out_path);
    let imgbuf = ImageBuffer::new_from_file(in_path);
    let tileset = make_tileset(&imgbuf);

    let tiling = generate_tiling(&tileset, w, h, seed);
    let mut out_buf = ImageBuffer::new(w*3,h*3);


    for tile_i in 0..w {
        for tile_j in 0..h {
            let tile = tileset.get_tilespec(tiling[tile_i * w + tile_j]);
            for i in 0..3 {
                for j in 0..3 {
                    let colour = tile.px_colour[i+j*3];
                    out_buf.set_px(tile_i * 3 + i, tile_j * 3 + j, colour);
                }
            }
        }
    }

    out_buf.dump_to_file(out_path);
}

fn rot_tile(pixel_data: [(u8, u8, u8); 9]) -> [(u8, u8, u8); 9] {
    let mut output_data = [(0, 0, 0); 9];

    for i in 0..3 {
        for j in 0..3 {
            output_data[i*3 + j] = pixel_data[(2-j)*3 + i];
        }
    }

    output_data
}

fn main() {
    
    do_tiles("test_tilesets/horz.png", "test_results/horz.png", 60, 60, 69);
    do_tiles("test_tilesets/vert.png", "test_results/vert.png", 60, 60, 69);
    do_tiles("test_tilesets/dontplace.png", "test_results/dontplace.png", 60, 60, 69);
    do_tiles("test_tilesets/dontplaceh.png", "test_results/dontplaceh.png", 60, 60, 69);
    do_tiles("test_tilesets/dontplace2.png", "test_results/dontplace2.png", 60, 60, 72);
    do_tiles("test_tilesets/rps.png", "test_results/rps.png", 60, 60, 72);
    do_tiles("test_tilesets/flower.png", "test_results/flower.png", 60, 60, 69);
    do_tiles("test_tilesets/flower2.png", "test_results/flower2.png", 60, 60, 69);
    do_tiles("test_tilesets/roads.png", "test_results/roads.png", 60, 60, 69);
    do_tiles("test_tilesets/pluroads.png", "test_results/pluroads.png", 60, 60, 69);
    do_tiles("test_tilesets/testbeach.png", "test_results/testbeach.png", 60, 60, 69);
    do_tiles("test_tilesets/testbeachgrass.png", "test_results/testbeachgrass.png", 60, 60, 69);
    do_tiles("test_tilesets/testbeachgrassforest.png", "test_results/testbeachgrassforest.png", 60, 60, 69);
    do_tiles("test_tilesets/testbeachgrassforest.png", "test_results/testbeachgrassforest2.png", 60, 60, 70);
    do_tiles("test_tilesets/testbeachgrassforest.png", "test_results/testbeachgrassforest3.png", 60, 60, 71);
    do_tiles("test_tilesets/testbeachgrassforest.png", "test_results/testbeachgrassforest4.png", 60, 60, 72);
    do_tiles("test_tilesets/testoilwater.png", "test_results/testoilwater.png", 60, 60, 69);
    do_tiles("test_tilesets/testvillage.png", "test_results/testvillage.png", 60, 60, 69);
    do_tiles("test_tilesets/testvillage.png", "test_results/testvillage2.png", 60, 60, 70);
    do_tiles("test_tilesets/testvillage.png", "test_results/testvillage3.png", 60, 60, 71);
    do_tiles("test_tilesets/testvillage.png", "test_results/testvillage4.png", 60, 60, 72);
}
