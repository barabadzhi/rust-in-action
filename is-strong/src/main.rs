fn main() {
    let pw = "justok";
    let is_strong = is_strong(pw);

    println!("Is strong? {}", is_strong)
}

fn is_strong<T: Into<String>>(password: T) -> bool {
    password.into().len() > 5
}
