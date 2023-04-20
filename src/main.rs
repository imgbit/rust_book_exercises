mod common_programming_concepts;

fn main() {
    println!(
        "common_programming_concepts::fahrenheit_to_celsius(100) = {}",
        common_programming_concepts::fahrenheit_to_celsius(100.0) as u32
    );

    println!(
        "common_programming_concepts::nth_fibonacci(5) = {}",
        common_programming_concepts::nth_fibonacci(10)
    );

    println!(
        "common_programming_concepts::write_a_christmas_carol() = \"{}\"",
        common_programming_concepts::write_a_christmas_carol()
    );
}
