pub fn format() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. There will be stringified.

    println!("{} days", 10);

    // Positional arguments can be used.
    println!("{0},this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("{number: >5}", number = 1);
}
