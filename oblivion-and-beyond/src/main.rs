fn main() {
    let mut i: u16 = 0;
    print!("{}..", i);

    loop {
        i += 100;
        print!("{}..", i);

        if i % 10_000 == 0 {
            println!();
        }
    }
}
