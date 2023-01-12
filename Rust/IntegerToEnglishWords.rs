use std::collections::HashMap;

struct Solution;


// Convert a non-negative integer num to its English words representation.
impl Solution {
    pub fn number_to_words(num: i32) -> String {
        fn process_digits(num: i32) -> &'static str {
            let digits = HashMap::from([
                (1, "One "),
                (2, "Two "),
                (3, "Three "),
                (4, "Four "),
                (5, "Five "),
                (6, "Six "),
                (7, "Seven "),
                (8, "Eight "),
                (9, "Nine ")
            ]);
            return digits.get(&num).unwrap_or(&"");
        }
        fn process_tens(num: i32) -> String {
            if num < 10 {
                return process_digits(num).to_owned()
            }
            let digits = HashMap::from([
                (10, "Ten "),
                (11, "Eleven "),
                (12, "Twelve "),
                (2, "Twen"),
                (3, "Thir"),
                (14, "Fourteen "),
                (4, "For"),
                (5, "Fif"),
                (8, "Eigh")
            ]);
            return if num < 20 {
                digits.get(&num)
                    .unwrap_or(&&**&(digits.get(&(num % 10))
                        .unwrap_or(&process_digits(num % 10)).trim_end().to_owned().to_owned() + "teen ")
                    ).to_owned().to_owned()
            } else {
                digits.get(&(num / 10)).unwrap_or(&process_digits(num / 10)).trim_end()
                    .to_owned().to_owned() + "ty " + &process_digits(num % 10)
            }
        }
        fn process_hundreds(num: i32) -> String {
            return if num == 0 {
                "".to_owned()
            } else if num >= 100 {
                process_digits(num / 100).to_owned().to_owned() + "Hundred " + &process_tens(num % 100)
            } else {
                process_tens(num)
            }
        }
        fn process_thousands(num: i32) -> String {
            return if num >= 1000 {
                process_hundreds(num / 1000).to_owned() + "Thousand " + &process_hundreds(num % 1000)
            } else {
                process_hundreds(num)
            }
        }
        fn process_millions(num: i32) -> String {
            return if num >= 1_000_000 {
                process_hundreds(num / 1_000_000).to_owned() + "Million " + &process_thousands(num % 1_000_000)
            } else {
                process_thousands(num % 1_000_000)
            }
        }
        fn process_billions(num: i32) -> String {
            return if num >= 1_000_000_000 {
                process_digits(num / 1_000_000_000).to_owned() + "Billion " + &process_millions(num % 1_000_000_000)
            } else {
                process_millions(num % 1_000_000_000)
            }
        }
        return if num == 0 {
            "Zero".to_owned()
        } else {
            process_billions(num).trim_end().to_owned()
        }
    }
}
