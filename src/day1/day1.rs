mod tools;

fn main() {
    println!("Day result: {}", day::process());
    println!("Bonus result: {}", bonus::process());
}

mod day {
    pub fn process() -> usize {
        if let Ok(values) = super::tools::read_values::<usize>("src/day1/values.txt") {
            for value in values.iter() {
                let expected_pair = 2020 - value;
                if let Some(found) = values.iter().find(|&&x| x == expected_pair) {
                    return value * found;
                }
            }
        }

        0
    }
}

mod bonus {
    pub fn process() -> i32 {
        if let Ok(mut values) = super::tools::read_values::<i32>("src/day1/values.txt") {
            while let Some(first_val) = values.pop() {
                for second_val in values.iter() {
                    let expected_third = 2020 - second_val - first_val;
                    if expected_third > 0 {
                        if let Some(found) = values.iter().find(|&&x| x == expected_third) {
                            return first_val * second_val * found;
                        }
                    }
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn day() {
        assert_eq!(super::day::process(), 605364);
    }

    #[test]
    fn bonus() {
        assert_eq!(super::bonus::process(), 128397680);
    }
}
