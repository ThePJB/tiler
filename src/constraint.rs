use assert_hex::*;
use crate::grid::*;

#[derive(Copy, Clone)]
pub struct Constraint {
    pub r: u64,
    pub g: u64,
    pub b: u64,
    pub mask: u64,
}

pub fn constraint_match(c1: Constraint, c2: Constraint) -> bool {
    let mask = c1.mask | c2.mask;
    c1.r | mask == c2.r | mask &&
    c1.g | mask == c2.g | mask &&
    c1.b | mask == c2.b | mask
}

#[test]
pub fn test_constraint_match() {
    let unconstrained_all = Constraint {
        r: 0,
        g: 0,
        b: 0,
        mask: 0xFFFFFFFFFFFFFFFF,
    };
    let unconstrained_bottom = Constraint {
        r: 0,
        g: 0,
        b: 0,
        mask: 0x0000000000FFFFFF,
    };
    let red_bottom = Constraint {
        r: 0x0000000000FFFFFF,
        g: 0,
        b: 0,
        mask: 0xFFFFFFFFFF000000,
    };
    let red_all = Constraint {
        r: 0xFFFFFFFFFFFFFFFF,
        g: 0,
        b: 0,
        mask: 0,
    };
    let magenta_bottom = Constraint {
        r: 0x0000000000FFFFFF,
        g: 0,
        b: 0x0000000000FFFFFF,
        mask: 0xFFFFFFFFFF000000,
    };
    assert_eq!(constraint_match(unconstrained_all, red_bottom), true);
    assert_eq!(constraint_match(unconstrained_all, red_all), true);
    assert_eq!(constraint_match(unconstrained_all, magenta_bottom), true);
    assert_eq!(constraint_match(unconstrained_bottom, magenta_bottom), true);
    assert_eq!(constraint_match(red_bottom, red_all), true);
    assert_eq!(constraint_match(red_bottom, magenta_bottom), false);

}

fn flip_ud(x: u64) -> u64 {
    // swap first 3 bytes with last 3 bytes
    (x & 0x000000FFFF000000) | (x >> 40) | (x << 40)
}

#[test]
fn test_flip_ud() {
    assert_eq_hex!(flip_ud(0xF000000000000000), 0x0000000000F00000);
    assert_eq_hex!(flip_ud(0x0000000000F00000), 0xF000000000000000);
    assert_eq_hex!(flip_ud(0x00000F0000000000), 0x000000000000000F);
    assert_eq_hex!(flip_ud(0x000000000000000F), 0x00000F0000000000);
    assert_eq_hex!(flip_ud(0xFFF0000000F00000), 0xF000000000FFF000);
}

fn flip_lr(x: u64) -> u64 {
    // swap left and right bytes
     x & 0x00FF00000000FF00 |
    (x & 0xFF00000000FF0000) >> 16 |
    (x & 0x0000FF00000000FF) << 16 |
    (x & 0x00000000FF000000) << 8  |
    (x & 0x000000FF00000000) >> 8
}

fn constraint_flip_ud(c: Constraint) -> Constraint {
    Constraint {
        r: flip_ud(c.r),
        g: flip_ud(c.g),
        b: flip_ud(c.b),
        mask: flip_ud(c.mask),
    }
}

fn constraint_flip_lr(c: Constraint) -> Constraint {
    Constraint {
        r: flip_lr(c.r),
        g: flip_lr(c.g),
        b: flip_lr(c.b),
        mask: flip_lr(c.mask),
    }
}

const DIR_MASK_SOUTH: u64 = 0xFFFFFF0000000000;
const DIR_MASK_NORTH: u64 = 0x0000000000FFFFFF;
const DIR_MASK_EAST: u64  = 0xFF0000FF00FF0000;
const DIR_MASK_WEST: u64  = 0x0000FF00FF0000FF;

pub fn constraint_add(target: &mut Constraint, source: Constraint, dir: Dir) {

    let (dir_mask, flipped_source) = match dir {
        Dir::North => (DIR_MASK_NORTH, constraint_flip_ud(source)),
        Dir::South => (DIR_MASK_SOUTH, constraint_flip_ud(source)),
        Dir::East => (DIR_MASK_EAST, constraint_flip_lr(source)),
        Dir::West => (DIR_MASK_WEST, constraint_flip_lr(source)),
    };

    target.mask &= !dir_mask;                        // zero the mask bits
    target.mask |= dir_mask & flipped_source.mask;   // add relevant source bits

    target.r &= !dir_mask;
    target.r |= dir_mask &flipped_source.r;

    target.g &= !dir_mask;
    target.g |= dir_mask &flipped_source.g;

    target.b &= !dir_mask;
    target.b |= dir_mask &flipped_source.b;
}

pub fn constraint_from_px_colour(px_colour: [(u8, u8, u8); 9]) -> Constraint {
    Constraint {
        r: 
            (px_colour[0].0 as u64) << 56 |
            (px_colour[1].0 as u64) << 48 |
            (px_colour[2].0 as u64) << 40 |
            (px_colour[3].0 as u64) << 32 |
            // skip middle one for constraints
            (px_colour[5].0 as u64) << 24 |
            (px_colour[6].0 as u64) << 16 |
            (px_colour[7].0 as u64) << 8 |
            (px_colour[8].0 as u64) << 0,
        g: 
            (px_colour[0].1 as u64) << 56 |
            (px_colour[1].1 as u64) << 48 |
            (px_colour[2].1 as u64) << 40 |
            (px_colour[3].1 as u64) << 32 |
            // skip middle one for constraints
            (px_colour[5].1 as u64) << 24 |
            (px_colour[6].1 as u64) << 16 |
            (px_colour[7].1 as u64) << 8 |
            (px_colour[8].1 as u64) << 0,
        b: 
            (px_colour[0].2 as u64) << 56 |
            (px_colour[1].2 as u64) << 48 |
            (px_colour[2].2 as u64) << 40 |
            (px_colour[3].2 as u64) << 32 |
            // skip middle one for constraints
            (px_colour[5].2 as u64) << 24 |
            (px_colour[6].2 as u64) << 16 |
            (px_colour[7].2 as u64) << 8 |
            (px_colour[8].2 as u64) << 0,
        mask: 0,
    }
}