use shim_instrument::shim_instrument;

fn main() {
    let a = f1(1);
    let b = f2("world");
    println!("{} {}", a, b);
}

#[shim_instrument(skip_all, level = "Info")]
fn f1(a: i32) -> i32 {
    a + 1
}

#[shim_instrument(level = "Debug")]
fn f2(a: &str) -> String {
    format!("Hello, {}", a)
}