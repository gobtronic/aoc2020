use std::ops::RangeInclusive;

mod tools;

fn main() {
    println!("Day result: {}", day::process(false));
    println!("Bonus result: {}", day::process(true));
}

struct Password {
    pub value: String,
    pub policy: PasswordPolicy,
}

struct PasswordPolicy {
    pub char: char,
    pub range: RangeInclusive<i32>,
}

impl Password {
    pub fn new(line: String) -> Password {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let mut parts_iter = parts.iter();
        let range = Password::extract_range(parts_iter.next());
        let char = Password::extract_char(parts_iter.next());
        let value = parts_iter.next().unwrap_or(&"");

        Password {
            value: value.to_string(),
            policy: PasswordPolicy { char, range },
        }
    }

    fn extract_range(str: Option<&&str>) -> RangeInclusive<i32> {
        let str = str.unwrap_or(&"99-99");
        let parts: Vec<&str> = str.split('-').collect();

        let start = parts.first().unwrap_or(&"99").parse::<i32>().unwrap_or(99);
        let end = parts.last().unwrap_or(&"99").parse::<i32>().unwrap_or(99);

        start..=end
    }

    fn extract_char(str: Option<&&str>) -> char {
        let str = str.unwrap_or(&"a:");
        str.chars().next().unwrap_or('a')
    }

    pub fn respects_policy(&self) -> bool {
        let expected_chars = self.value.chars().filter(|char| *char == self.policy.char);
        self.policy.range.contains(&(expected_chars.count() as i32))
    }

    pub fn respects_advanced_policy(&self) -> bool {
        let chars: Vec<char> = self.value.chars().collect();
        let first_char = chars.get((self.policy.range.start().abs() - 1) as usize);
        let second_char = chars.get((self.policy.range.end().abs() - 1) as usize);
        match (first_char, second_char) {
            (Some(first), Some(second)) => {
                let chars = [first, second];
                return chars
                    .iter()
                    .filter(|char| ***char == self.policy.char)
                    .count()
                    == 1;
            }
            _ => false,
        }
    }
}

mod day {
    use crate::Password;

    pub fn process(bonus: bool) -> usize {
        if let Ok(values) = super::tools::read_values::<String>("src/day2/values.txt") {
            let good_pws = values.iter().filter(|value| {
                let password = Password::new(value.to_string());
                if bonus {
                    password.respects_advanced_policy()
                } else {
                    password.respects_policy()
                }
            });

            return good_pws.count();
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use crate::Password;

    #[test]
    fn respects_policy() {
        let password = Password {
            value: "foobar".to_string(),
            policy: crate::PasswordPolicy {
                char: 'o',
                range: 1..=5,
            },
        };
        assert!(password.respects_policy());

        let password = Password {
            value: "foobar".to_string(),
            policy: crate::PasswordPolicy {
                char: 'o',
                range: 3..=5,
            },
        };
        assert!(!password.respects_policy());

        let password = Password {
            value: "fobaroo".to_string(),
            policy: crate::PasswordPolicy {
                char: 'o',
                range: 3..=5,
            },
        };
        assert!(password.respects_policy());

        let password = Password {
            value: "fobaroooo".to_string(),
            policy: crate::PasswordPolicy {
                char: 'o',
                range: 3..=5,
            },
        };
        assert!(password.respects_policy());

        let password = Password {
            value: "fobaroorororororo".to_string(),
            policy: crate::PasswordPolicy {
                char: 'o',
                range: 3..=5,
            },
        };
        assert!(!password.respects_policy());
    }

    #[test]
    fn respects_advanced_policy() {
        let password = Password {
            value: "foobar".to_string(),
            policy: crate::PasswordPolicy {
                char: 'o',
                range: 2..=3,
            },
        };
        assert!(!password.respects_advanced_policy());

        let password = Password {
            value: "fooobar".to_string(),
            policy: crate::PasswordPolicy {
                char: 'o',
                range: 2..=5,
            },
        };
        assert!(password.respects_advanced_policy());
    }

    #[test]
    fn make_policy() {
        let password = Password::new("9-10 k: vfbbmbxwkd".to_string());
        assert_eq!(password.value, "vfbbmbxwkd");
        assert_eq!(password.policy.range, 9..=10);
        assert_eq!(password.policy.char, 'k');
    }

    #[test]
    fn day() {
        assert_eq!(super::day::process(false), 342);
    }

    #[test]
    fn bonus() {
        assert_eq!(super::day::process(true), 745);
    }
}
