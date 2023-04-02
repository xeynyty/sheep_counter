fn main() {
    let mut count: u128 = 0;
    loop {
        count = count.saturating_add(1);
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("{} sheep", &count);
        if count == u128::MAX {
            println!("Have you counted all the sheep!");
            break
        }
    }
}