fn main() {
    println!("max of input range: {:08b} -> {}", 0xff, mock_rand(0xff));
    println!("mid of input range: {:08b} -> {}", 0x77, mock_rand(0x77));
    println!("min of input range: {:08b} -> {}", 0x00, mock_rand(0x00));
}

fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0011_1111_0000_0000_0000_0000_0000_0000;
    let large_n = u32::from(n) << 15;
    let f32_bits = base | large_n;
    let m = f32::from_bits(f32_bits);
    2_f32 * (m - 0.5)
}
