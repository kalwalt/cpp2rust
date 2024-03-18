#[link(name = "main")]

// src/main.rs
extern "C" {
    fn UniqueFunction(a: i32, b: i32) -> bool;
}

fn main() {
    unsafe {
        println!("Value: {}", UniqueFunction(23, 4));
    }
}

