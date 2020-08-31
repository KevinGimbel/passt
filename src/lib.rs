use std::fs::File;
use std::io::Read;

// default character set without special characters
pub const CHARS_NO_SPECIAL: &str = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZZYXWVUTSRQPONMLKJIHGFEDCBA9876543210zyxwvutsrqponmlkjihgfedcba";
// default character set with special characters
pub const CHARS_SPECIAL: &str =
    "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ!§$%&/()=?´`-_.,:;#'+*<>°^^°><*+'#;:,._-`´?=)(&%$§!ZYXWVUTSRQPONMLKJIHGFEDCBA9876543210zyxwvutsrqponmlkjihgfedcba";

pub struct Passt {}

impl Passt {
    // return a random character from a set of characters specified as &str
    pub fn choose_random_char(char_set: &str) -> String {
        // not tested!
        #[cfg(target_os = "windows")]
        let mut f = File::open("file:/dev/urandom").unwrap();
        #[cfg(not(target_os = "windiws"))]
        let mut f = File::open("/dev/urandom").unwrap();
        let mut buf = [0u8; 32];
        f.read_exact(&mut buf).unwrap();

        let char_len_i64 = char_set.len() as i64;
        // Add together all 32 random numbers we read from /dev/urandom
        let mut total: i64 = 0;
        for n in buf.iter() {
            total = (total + (*n as i64)) * 2;
        }
        // whatever this is, it does work and selects a index within range 🤷‍♀️
        let mut index: i64 = total % (char_len_i64) / 2;
        if index > char_len_i64 {
            index = index / char_len_i64;
        }
        let mut ret = String::from("");
        let mut idx = 0;
        // we need to extract chars because some "letters" are more than 1 byte long, e.g. `ä` or `ß`,
        // if we collect them into a Vec<str> and then index, we could get errors when hitting any special char.
        for c in char_set.chars() {
            ret = format!("{}", c);
            idx = idx + 1;
            if idx == index {
                break;
            }
        }

        // In case we have an empty string, just use the first character
        // Again, this is "good enough"
        if ret == String::from("") {
            //char_set.chars().next().unwrap().to_string();
            String::from("A");
        }

        return ret.to_string();
    }

    // generate random password with length and optional special characters included
    pub fn random_password(length: i32, with_special_chars: Option<bool>) -> String {
        // Check if with_special_chars is set and if it is set, if it's true or false
        let char_set: &str = match with_special_chars {
            Some(use_special) => {
                if use_special {
                    CHARS_SPECIAL
                } else {
                    CHARS_NO_SPECIAL
                }
            }
            _ => CHARS_NO_SPECIAL,
        };

        let pass = Self::random_password_with_custom_set(length, char_set);

        return pass;
    }

    pub fn random_password_with_custom_set(length: i32, char_set: &str) -> String {
        let mut len = length;
        let len_usize = length as usize;
        let mut pass = String::from("");

        // add characters to the string until it is filled
        while len > 0 {
            pass = format!("{}{}", pass, Passt::choose_random_char(char_set));
            len = len - 1;
        }

        // If password is too small, add more characters
        if pass.len() != len_usize {
            while pass.len() < len_usize {
                pass = format!("{}{}", pass, Passt::choose_random_char(char_set));
            }
        };

        // Make sure string never exceeds the desired length
        pass.chars().take(len_usize).collect()
    }
}

mod test {
    #[test]
    fn test_random_password() {
        use super::*;
        use std::collections::HashMap;
        // cases has the following format
        // HashMap<usize, usize>
        // HashMap<ExpectedLength, CalculatedPasswordLength>
        let mut cases: HashMap<usize, usize> = HashMap::new();
        cases.insert(12, Passt::random_password(12, Some(false)).chars().count());
        cases.insert(18, Passt::random_password(18, None).chars().count());
        cases.insert(36, Passt::random_password(36, Some(true)).chars().count());
        cases.insert(
            125,
            Passt::random_password(125, Some(false)).chars().count(),
        );
        cases.insert(2, Passt::random_password(2, Some(true)).chars().count());
        cases.insert(255, Passt::random_password(255, Some(true)).chars().count());

        for case in cases.iter() {
            assert_eq!(case.0, case.1);
        }
    }
    #[test]
    fn test_random_password_with_custom_set() {
        use super::*;
        use std::collections::HashMap;
        // cases has the following format
        // HashMap<usize, usize>
        // HashMap<ExpectedLength, CalculatedPasswordLength>
        let mut cases: HashMap<usize, usize> = HashMap::new();
        cases.insert(
            12,
            Passt::random_password_with_custom_set(
                12,
                "oqeqkobhwrio0ß3ri249ßt9ßjskfbnkfpfbneob?)()&/(/!wkvhoqvkpwnbö56185d465q4wf646ew",
            )
            .chars()
            .count(),
        );
        cases.insert(
            18,
            Passt::random_password_with_custom_set(18, "hs")
                .chars()
                .count(),
        );
        cases.insert(
            36,
            Passt::random_password_with_custom_set(
                36,
                "qwifqpehveqpviqpevqepvbqivakjfiowhet9823ur9ß13rhfeiqwafjvdsiohg2iw",
            )
            .chars()
            .count(),
        );
        cases.insert(
            125,
            Passt::random_password_with_custom_set(
                125,
                "qwifqpehveqpviqpevqepvbqivakjfiowhet9823ur9ß13rhfeiqwafjvdsiohg2iw",
            )
            .chars()
            .count(),
        );
        cases.insert(
            2,
            Passt::random_password_with_custom_set(
                2,
                "qwifqpehveqpviqpevqepvbqivakjfiowhet9823ur9ß13rhfeiqwafjvdsiohg2iw",
            )
            .chars()
            .count(),
        );
        cases.insert(
            12,
            Passt::random_password_with_custom_set(12, "0123456789abc")
                .chars()
                .count(),
        );
        cases.insert(
            6,
            Passt::random_password_with_custom_set(6, "äöüÄÖÜáéóúÁÉÓÚàèòùÀÈÒÙß$§")
                .chars()
                .count(),
        );
        cases.insert(
            7,
            Passt::random_password_with_custom_set(7, "abc")
                .chars()
                .count(),
        );
        cases.insert(
            7,
            Passt::random_password_with_custom_set(7, "🤷‍♀️🥨🥇")
                .chars()
                .count(),
        );

        for case in cases.iter() {
            assert_eq!(case.0, case.1);
        }
    }
    #[test]
    fn test_1000_random_passwords() {
        use super::*;

        let mut i = 0;
        let max = 10000;
        while i < max {
            i = i + 1;
            let expected: i32 = max / i;
            assert_eq!(
                expected,
                Passt::random_password(expected, Some(false))
                    .chars()
                    .count() as i32
            );
        }
    }
    #[test]
    fn test_1000_random_passwords_with_special() {
        use super::*;
        let mut i = 0;
        let max = 1000;
        while i < max {
            i = i + 1;
            let expected: i32 = max / i;
            assert_eq!(
                expected,
                Passt::random_password(expected, Some(true)).chars().count() as i32
            );
        }
    }
    #[test]
    fn test_10000_random_passwords() {
        use super::*;
        let mut i = 0;
        let max = 10000;
        while i < max {
            i = i + 1;
            let expected: i32 = max / i;
            assert_eq!(
                expected,
                Passt::random_password(expected, Some(true)).chars().count() as i32
            );
        }
    }
    #[test]
    fn test_10000_random_password_with_special() {
        use super::*;
        let mut i = 0;
        let max = 10000;
        while i < max {
            i = i + 1;
            let expected: i32 = max / i;
            assert_eq!(
                expected,
                Passt::random_password(expected, Some(true)).chars().count() as i32
            );
        }
    }
}
