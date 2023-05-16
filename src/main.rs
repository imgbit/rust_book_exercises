mod common_collections;
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

    let array = [1, 2, 3, 4, 5];
    println!(
        "common_collections::median({:?}) = {}",
        array,
        common_collections::median(&array).unwrap()
    );

    let array = [1, 2, 3, 4, 4];
    println!(
        "common_collections::mode({:?}) = {}",
        array,
        common_collections::mode(&array).unwrap()
    );
}
