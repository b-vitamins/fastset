use std::collections::HashSet;

fn main() {
    let max_element = 100_000_000;
    let sparse_factor = 1;

    // Initialize the Set
    let mut set = HashSet::new();

    // Populate the set with sparse data
    for i in (0..max_element).step_by(sparse_factor) {
        set.insert(i);
    }

    // Perform some operations to simulate usage
    for i in (0..max_element).step_by(sparse_factor * 2) {
        assert!(set.contains(&i));
    }

    for i in (0..max_element).step_by(sparse_factor * 3) {
        set.remove(&i);
    }

    // Keep the binary from exiting immediately in release mode optimizations
    println!("Finished processing. Press ENTER to exit.");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}