fn main() {
    mutable();
    constants();
    error();
}

fn error() {
    let x = 5; // let is immutable by default
    x=6;
}

fn mutable() {
    let mut x = 5; // mut makes it mutable
    x = 6;
}

fn constants() {
    const HOUR_IN_SECONDS: u32 = 60 * 60;
    println!("{}", HOUR_IN_SECONDS);
}