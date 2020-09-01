fn main() {
    let mut a = 0;
    while a < 10 {
        println!("<{}>", a);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        a += 1;
    }
}
