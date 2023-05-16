fn is_power_of_two(num: u32) -> bool {
    if num == 0 {
        return false;
    }
    num & (num - 1) == 0
}

// Example usage:
fn main() {
    let numbers = [1, 2, 4, 8, 16, 32, 64, 128, 255];

    for &num in &numbers {
        if is_power_of_two(num) {
            println!("{} is a power of two.", num);
        } else {
            println!("{} is not a power of two.", num);
        }
    }
}
