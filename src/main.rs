mod day1;

fn main() {
    let day1_ret = day1::run();
    println!("Day 1: {}", day1_ret);
}

#[cfg(test)]
mod tests {
    use crate::day1;
    #[test]
    fn day1_test() {
        assert_eq!(day1::fuel_for_module(12), 2);
        assert_eq!(day1::fuel_for_module(14), 2);
        assert_eq!(day1::fuel_for_module(1969), 654);
        assert_eq!(day1::fuel_for_module(100756), 33583);
    }
}
