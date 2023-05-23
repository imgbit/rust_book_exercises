use std::collections::HashMap;

pub fn median(array: &[i32]) -> Option<f32> {
    if array.is_empty() {
        return None;
    }

    let mut vec = Vec::from(&array[..]);
    vec.sort();

    let length = vec.len();
    let middle = length / 2;

    if length % 2 == 0 {
        let median = (vec[middle - 1] + vec[middle]) / 2;
        Some(median as f32)
    } else {
        Some(vec[middle] as f32)
    }
}

pub fn mode(array: &[i32]) -> Option<i32> {
    if array.is_empty() {
        return None;
    }

    let mut hashmap = HashMap::new();
    // if all items occur just once, by default take the first one
    let mut max_value_cnt = 1;
    let mut max_value = &array[0];

    for num in array {
        let count = hashmap.entry(num).or_insert(0);
        *count += 1;

        if max_value_cnt < *count {
            max_value = &num;
            max_value_cnt = *count;
        }
    }

    Some(*max_value)
}

pub fn to_pig_latin(text: &str) -> String {
    let mut retstr = String::from("");
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    for word in text.split_whitespace() {
        let mut chars = word.chars();
        let first_char = &chars.next().unwrap();
        let rest_of_string: String = chars.collect();
        if vowels.contains(&first_char.to_lowercase().next().unwrap()) {
            retstr = format!("{} {}-hay", retstr, word);
        } else {
            if word.len() == 1 {
                retstr = format!("{} {}ay", retstr, first_char);
            } else {
                retstr = format!("{} {}-{}ay", retstr, rest_of_string, first_char);
            }
        }
    }

    retstr.trim_start_matches(' ').to_string()
}

#[derive(Default)]
pub struct EmployeesRecord {
    record: HashMap<String, Vec<String>>,
}

impl EmployeesRecord {
    pub fn len(&self) -> usize {
        self.record.len()
    }

    pub fn add_employee(&mut self, name: &str, department: &str) -> bool {
        if name.len() <= 0 || department.len() <= 0 {
            return false;
        }

        self.record
            .entry(department.to_string())
            .or_insert_with(|| Vec::new())
            .push(name.to_string());

        true
    }

    pub fn department_employees(&self, department: &str) -> Option<Vec<String>> {
        let value = self.record.get(department);

        match value {
            Some(v) => {
                let sortedvec = {
                    let mut vecclone = v.clone();
                    vecclone.sort();
                    vecclone
                };
                return Some(sortedvec);
            }
            None => return None,
        }
    }

    pub fn all_employees(&self) -> Option<Vec<String>> {
        let mut retvec: Vec<String> = vec![];
        for (_, value) in &self.record {
            for v in value {
                retvec.push(v.clone());
            }
        }

        retvec.sort();
        Some(retvec)
    }
}

impl std::fmt::Display for EmployeesRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "EmployeesRecord:")?;

        for (key, value) in &self.record {
            let sortedvec = {
                let mut vecclone = value.clone();
                vecclone.sort();
                vecclone
            };

            writeln!(f, "   {}:", key)?;
            for v in sortedvec {
                writeln!(f, "       {}", v)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employees_record() {
        let mut er: EmployeesRecord = Default::default();
        assert_eq!(er.add_employee("Joe", "Sales"), true);
        assert_eq!(er.add_employee("John", "Sales"), true);
        assert_eq!(er.add_employee("Jake", "Sales"), true);
        assert_eq!(er.add_employee("Mia", "HR"), true);
        assert_eq!(er.add_employee("Molly", "HR"), true);
        assert_eq!(er.add_employee("Megan", "HR"), true);
        assert_eq!(er.len(), 2);

        let employees = er.department_employees("HR").unwrap();
        assert_eq!(employees.len(), 3);
        // should be sorted alphabetically
        assert_eq!(employees[0], "Megan".to_string());
        assert_eq!(employees[1], "Mia".to_string());
        assert_eq!(employees[2], "Molly".to_string());

        let employees = er.department_employees("Sales").unwrap();
        assert_eq!(employees.len(), 3);
        // should be sorted alphabetically
        assert_eq!(employees[0], "Jake".to_string());
        assert_eq!(employees[1], "Joe".to_string());
        assert_eq!(employees[2], "John".to_string());

        assert_eq!(er.add_employee("", "B"), false);
        assert_eq!(er.add_employee("A", ""), false);

        let employees = er.all_employees().unwrap();
        assert_eq!(employees.len(), 6);
        // should be sorted alphabetically
        assert_eq!(employees[0], "Jake".to_string());
        assert_eq!(employees[1], "Joe".to_string());
        assert_eq!(employees[2], "John".to_string());
        assert_eq!(employees[3], "Megan".to_string());
        assert_eq!(employees[4], "Mia".to_string());
        assert_eq!(employees[5], "Molly".to_string());
    }

    #[test]
    fn test_median() {
        assert_eq!(median(&[1, 2, 3, 4, 5]).unwrap(), 3 as f32);
        assert_eq!(median(&[-1, 0, 2, -4, 5, 1, 1]).unwrap(), 1 as f32);
        assert_eq!(median(&[1]).unwrap(), 1 as f32);
        assert_eq!(median(&[-1, 1]).unwrap(), 0 as f32);
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn test_mode() {
        assert_eq!(mode(&[1, 1, 3, 4, 5]).unwrap(), 1);
        assert_eq!(mode(&[1, 2, 3, 4, 5]).unwrap(), 1);
        assert_eq!(mode(&[2, 2, 3, 4, 5]).unwrap(), 2);
        assert_eq!(mode(&[2, 2, 2, 1, 1, 1]).unwrap(), 2);
        assert_eq!(mode(&[2, 2, 2, 1, 1, 1, 1]).unwrap(), 1);
        assert_eq!(mode(&[-1, -1, 3, 4, 5]).unwrap(), -1);
        assert_eq!(mode(&[2]).unwrap(), 2);
        assert_eq!(mode(&[2, 2]).unwrap(), 2);
        assert_eq!(mode(&[]), None);
    }

    #[test]
    fn test_to_pig_latin() {
        assert_eq!(to_pig_latin(&String::from("this is a string containing a few words and will be translated into pig Latin")),
        "his-tay is-hay a-hay tring-say ontaining-cay a-hay ew-fay ords-way and-hay ill-way e-bay ranslated-tay into-hay ig-pay atin-Lay".to_string());
        assert_eq!(to_pig_latin(&String::from("")), "".to_string());
        assert_eq!(to_pig_latin(&String::from("1")), "1ay".to_string());
        assert_eq!(to_pig_latin(&String::from("A")), "A-hay".to_string());
        assert_eq!(to_pig_latin(&String::from("B")), "Bay".to_string());
        assert_eq!(to_pig_latin(&String::from("a")), "a-hay".to_string());
        assert_eq!(to_pig_latin(&String::from("b")), "bay".to_string());
        assert_eq!(to_pig_latin(&String::from("aa")), "aa-hay".to_string());
        assert_eq!(to_pig_latin(&String::from("bb")), "b-bay".to_string());
        assert_eq!(
            to_pig_latin(&String::from("aa aa")),
            "aa-hay aa-hay".to_string()
        );
        assert_eq!(
            to_pig_latin(&String::from("bb bb")),
            "b-bay b-bay".to_string()
        );
        assert_eq!(
            to_pig_latin(&String::from("aa bb")),
            "aa-hay b-bay".to_string()
        );
        assert_eq!(
            to_pig_latin(&String::from("bb aa")),
            "b-bay aa-hay".to_string()
        );
        assert_eq!(
            to_pig_latin(&String::from("aa bb aa")),
            "aa-hay b-bay aa-hay".to_string()
        );
        assert_eq!(
            to_pig_latin(&String::from("bb aa bb")),
            "b-bay aa-hay b-bay".to_string()
        );
        assert_eq!(
            to_pig_latin(&String::from("Κάβουρας")),
            "άβουρας-Κay".to_string()
        );
        assert_eq!(to_pig_latin(&String::from("螃蟹")), "蟹-螃ay".to_string());
    }
}
