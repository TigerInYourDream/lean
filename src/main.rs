pub fn main() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Hello, world!");
    }
}
