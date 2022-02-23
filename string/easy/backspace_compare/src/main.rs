fn backspace_compare(str1: String, str2: String) -> bool {
    let type_out_str_1 = type_out(str1);
    let type_out_str_2 = type_out(str2);

    return type_out_str_1 == type_out_str_2;
}

fn type_out(str: String) -> String {
    let mut tmp_str = String::new();
    for c in str.bytes() {
        if c == b'#' {
            tmp_str.pop();
        } else {
            tmp_str.push(c as char)
        }
    }
    return tmp_str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backspace_compare() {
        struct DataTest {
            str1: String,
            str2: String,
            expected_output: bool,
        }

        let tests: [DataTest; 5] = [
            DataTest {
                str1: String::from("abc#"),
                str2: String::from("ab"),
                expected_output: true,
            },
            DataTest {
                str1: String::from("abc##"),
                str2: String::from("a"),
                expected_output: true,
            },
            DataTest {
                str1: String::from("bxj##tw"),
                str2: String::from("bxo#j##tw"),
                expected_output: true,
            },
            DataTest {
                str1: String::from("bxj##tw"),
                str2: String::from("bxo###tw"),
                expected_output: false,
            },
            DataTest {
                str1: String::from("a#c"),
                str2: String::from("b"),
                expected_output: false,
            },
        ];

        for test in tests {
            let real_output = backspace_compare(test.str1, test.str2);
            assert_eq!(real_output, test.expected_output);
        }
    }
}
