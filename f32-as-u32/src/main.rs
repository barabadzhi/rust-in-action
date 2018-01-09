fn main() {
    let a: f32 = 42.42;
    let frankentype: u32 = unsafe { std::mem::transmute(a) };

    println!("{:032b}", frankentype);
}
