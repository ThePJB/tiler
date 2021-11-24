pub fn khash(seed: u32) -> u32 {
    let n1 = 0xB5297A4D;
    let n2 = 0x68E31DA4;
    let n3 = 0x1B56C4E9;

    let mut mangled = seed;
    mangled = mangled.wrapping_mul(n1);
    mangled ^= mangled.rotate_right(13);
    mangled = mangled.wrapping_add(n2);
    mangled ^= mangled.rotate_left(7);
    mangled = mangled.wrapping_mul(n3);
    mangled ^= mangled.rotate_right(9);
    return mangled;
}

// 0..1
pub fn uniform_f32(seed: u32) -> f32 {
    khash(seed) as f32 / std::u32::MAX as f32
}