extern "C" {
    fn adder(x: i32, y: i32) -> i32;
}

fn main() {
    unsafe {
        println!("{}", adder(1,3));
    }
}
