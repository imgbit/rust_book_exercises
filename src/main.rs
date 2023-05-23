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

    let plstr = String::from(
        "this is a string containing a few words and will be translated into pig Latin",
    );
    println!(
        "common_collections::to_pig_latin({}) = \"{}\"",
        &plstr,
        common_collections::to_pig_latin(&plstr)
    );

    let mut er: common_collections::EmployeesRecord = Default::default();
    er.add_employee("Joe", "Sales");
    er.add_employee("John", "Sales");
    er.add_employee("Jake", "Sales");
    er.add_employee("Mia", "HR");
    er.add_employee("Molly", "HR");
    er.add_employee("Megan", "HR");
    println!(
        "common_collections::EmployeesRecord:\n{}\nnumber of departments: {}\nemployees in HR: {:?}\nall employees: {:?}",
        er.to_string(), er.len(), er.department_employees("HR").unwrap(), er.all_employees().unwrap());
}
