fn main() {
    let a: u16 = 0b1100_0011_1100_0011;

    // compiler warn about the overflow ^_^
    #[allow(overflowing_literals)]
    let b: i16 = 0b1100_0011_1100_0011;

    println!("a: {}", a);
    println!("b: {}", b);
}
