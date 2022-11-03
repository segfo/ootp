use crate::constants::{DEFAULT_ALGORITHM, DEFAULT_BREADTH, DEFAULT_COUNTER, DEFAULT_DIGITS};
use hmacsha::{HmacSha, ShaTypes};

/// Convert a `u64` value to an array of 8 elements of 8-bit.
const fn u64_to_8_length_u8_array(input: u64) -> [u8; 8] {
    input.to_be_bytes()
}

fn make_opt(secret: &[u8], digits: u32, counter: u64, algorithm: &ShaTypes) -> String {
    let counter_bytes = u64_to_8_length_u8_array(counter);
    let mut hash = HmacSha::new(secret, &counter_bytes, algorithm);
    let digest = hash.compute_digest();
    let offset = usize::from(digest.last().unwrap() & 0xf);
    let value = (u32::from(digest[offset]) & 0x7f) << 24
        | (u32::from(digest[offset + 1]) & 0xff) << 16
        | (u32::from(digest[offset + 2]) & 0xff) << 8
        | (u32::from(digest[offset + 3]) & 0xff);
    let mut code = (value % 10_u32.pow(digits)).to_string();

    // Check whether the code is digits bits long, if not, use "0" to fill in the front
    if code.len() != (digits as usize) {
        code = "0".repeat((digits - (code.len() as u32)) as usize) + &code;
    }

    code
}

/// The Options for the HOTP `make` function.
#[derive(Clone, Copy)]
pub enum MakeOption<'a> {
    /// The default case. `Counter = 0` and `Digits = 6`.
    Default,
    /// Specify the `Counter`.
    Counter(u64),
    /// Specify the desired number of `Digits`.
    Digits(u32),
    /// Specify the SHA algorihm
    Algorithm(&'a ShaTypes),
    /// Specify both the `Counter` and the desired number of `Digits`.
    Full {
        counter: u64,
        digits: u32,
        algorithm: &'a ShaTypes,
    },
}

/// The Options for the HOTP and TOTP `check` function.
#[derive(Clone, Copy)]
pub enum CheckOption<'a> {
    /// The default case. `Counter = 0` and `Breadth = 0`.
    Default,
    /// Specify the `Counter`.
    Counter(u64),
    /// Specify the desired number of digits.
    Breadth(u64),
    /// Specify both the `Counter`, the desired `Breadth` and the `Algorithm`.
    Full {
        counter: u64,
        breadth: u64,
        algorithm: &'a ShaTypes,
    },
    /// Specify the SHA algorihm
    Algorithm(&'a ShaTypes),
}

/// The HOTP is a HMAC-based one-time password algorithm.
///
/// It takes one parameter, the shared secret between client and server.
pub struct Hotp {
    secret: Vec<u8>,
}

impl Hotp {
    pub const fn new(secret: Vec<u8>) -> Self {
        Self { secret }
    }

    /**
    Returns the one-time password as a `String`

    # Example #1

    ```
    use ootp::hotp::{Hotp, MakeOption};

    let hotp = Hotp::new("A strong shared secret".as_bytes().to_vec());
    let code = hotp.make(MakeOption::Default);
    ```

    # Example #2

    ```
    use ootp::hotp::{Hotp, MakeOption};
    let hotp = Hotp::new("A strong shared secret".as_bytes().to_vec());
    let code = hotp.make(MakeOption::Digits(8));
    ```

    # Example #3

    ```
    use ootp::hotp::{Hotp, MakeOption};
    use ootp::hmacsha::ShaTypes;
    let hotp = Hotp::new("A strong shared secret".as_bytes().to_vec());
    let code = hotp.make(MakeOption::Algorithm(&ShaTypes::Sha2_256));
    ```
    */
    pub fn make(&self, options: MakeOption) -> String {
        match options {
            MakeOption::Default => make_opt(
                &self.secret(),
                DEFAULT_DIGITS,
                DEFAULT_COUNTER,
                DEFAULT_ALGORITHM,
            ),
            MakeOption::Counter(counter) => {
                make_opt(&self.secret(), DEFAULT_DIGITS, counter, DEFAULT_ALGORITHM)
            }
            MakeOption::Digits(digits) => {
                make_opt(&self.secret(), digits, DEFAULT_COUNTER, DEFAULT_ALGORITHM)
            }
            MakeOption::Full {
                counter,
                digits,
                algorithm,
            } => make_opt(&self.secret(), digits, counter, algorithm),
            MakeOption::Algorithm(algorithm) => {
                make_opt(&self.secret(), DEFAULT_DIGITS, DEFAULT_COUNTER, algorithm)
            }
        }
    }
    /**
    Returns a boolean indicating if the one-time password is valid.

    # Example #1

    ```
    use ootp::hotp::{Hotp, MakeOption, CheckOption};

    let hotp = Hotp::new("A strong shared secret".as_bytes().to_vec());
    let code = hotp.make(MakeOption::Default);
    let check = hotp.check(code.as_str(), CheckOption::Default);
    ```

    # Example #2

    ```
    use ootp::hotp::{Hotp, MakeOption, CheckOption};
    let hotp = Hotp::new("A strong shared secret".as_bytes().to_vec());
    let code = hotp.make(MakeOption::Counter(2));
    let check = hotp.check(code.as_str(), CheckOption::Counter(2));
    ```
    */

    pub fn check(&self, otp: &str, options: CheckOption) -> bool {
        let (counter, breadth, algorithm) = match options {
            CheckOption::Default => (DEFAULT_COUNTER, DEFAULT_BREADTH, DEFAULT_ALGORITHM),
            CheckOption::Counter(counter) => (counter, DEFAULT_BREADTH, DEFAULT_ALGORITHM),
            CheckOption::Breadth(breadth) => (DEFAULT_COUNTER, breadth, DEFAULT_ALGORITHM),
            CheckOption::Full {
                counter,
                breadth,
                algorithm,
            } => (counter, breadth, algorithm),
            CheckOption::Algorithm(algorithm) => (DEFAULT_COUNTER, DEFAULT_BREADTH, algorithm),
        };
        for i in (counter - breadth)..=(counter + breadth) {
            let code = self.make(MakeOption::Full {
                counter: i,
                digits: otp.len() as u32,
                algorithm,
            });
            if code == otp {
                return true;
            }
        }
        false
    }

    /// Get a reference to the hotp's  secret.
    pub fn secret(&self) -> Vec<u8> {
        self.secret.clone()
    }
}

#[cfg(test)]
mod tests {
    use hmacsha::ShaTypes;

    use super::{u64_to_8_length_u8_array, CheckOption, Hotp, MakeOption};
    use crate::constants::DEFAULT_ALGORITHM;

    #[test]
    fn make_test() {
        let hotp = Hotp::new("A strong shared secret".as_bytes().to_vec());
        let code1 = hotp.make(MakeOption::Default);
        let code2 = hotp.make(MakeOption::Default);
        assert_eq!(code1, code2);
    }

    #[test]
    fn make_test_sha2() {
        let hotp = Hotp::new("A strong shared secret".as_bytes().to_vec());
        let code1 = hotp.make(MakeOption::Algorithm(&ShaTypes::Sha2_256));
        let code2 = hotp.make(MakeOption::Algorithm(&ShaTypes::Sha2_256));
        assert_eq!(code1, code2);
    }
    /// Taken from [RFC 4226](https://datatracker.ietf.org/doc/html/rfc4226#appendix-D)
    #[test]
    fn make_test_correcteness() {
        use hex;

        let secret = "12345678901234567890".as_bytes().to_vec();
        let hotp = Hotp::new(secret.clone());
        let hex_string = hex::encode(secret);
        assert_eq!(
            format!("0x{}", hex_string),
            "0x3132333435363738393031323334353637383930"
        );
        let code = hotp.make(MakeOption::Counter(0));
        assert_eq!(code, "755224");
        let code = hotp.make(MakeOption::Counter(1));
        assert_eq!(code, "287082");
        let code = hotp.make(MakeOption::Counter(2));
        assert_eq!(code, "359152");
        let code = hotp.make(MakeOption::Counter(3));
        assert_eq!(code, "969429");
        let code = hotp.make(MakeOption::Counter(4));
        assert_eq!(code, "338314");
        let code = hotp.make(MakeOption::Counter(5));
        assert_eq!(code, "254676");
        let code = hotp.make(MakeOption::Counter(6));
        assert_eq!(code, "287922");
        let code = hotp.make(MakeOption::Counter(7));
        assert_eq!(code, "162583");
        let code = hotp.make(MakeOption::Counter(8));
        assert_eq!(code, "399871");
        let code = hotp.make(MakeOption::Counter(9));
        assert_eq!(code, "520489");
    }

    #[test]
    fn check_test() {
        let hotp = Hotp::new("A strong shared secret".as_bytes().to_vec());
        let code = hotp.make(MakeOption::Default);
        let check = hotp.check(code.as_str(), CheckOption::Default);
        assert!(check);
    }

    #[test]
    fn check_test_sha2() {
        let hotp = Hotp::new("A strong shared secret".as_bytes().to_vec());
        let code = hotp.make(MakeOption::Algorithm(&ShaTypes::Sha2_256));
        let check = hotp.check(code.as_str(), CheckOption::Algorithm(&ShaTypes::Sha2_256));
        assert!(check);
    }

    #[test]
    fn check_test_counter() {
        let hotp = Hotp::new("A strong shared secret".as_bytes().to_vec());
        let code = hotp.make(MakeOption::Counter(42));
        let check = hotp.check(code.as_str(), CheckOption::Counter(42));
        assert!(check);
    }

    #[test]
    fn check_test_breadth() {
        let hotp = Hotp::new("A strong shared secret".as_bytes().to_vec());
        let code = hotp.make(MakeOption::Counter(42));
        let check = hotp.check(
            code.as_str(),
            CheckOption::Full {
                counter: 42,
                breadth: 6,
                algorithm: DEFAULT_ALGORITHM,
            },
        );
        assert!(check);
    }

    #[test]
    fn check_test_counter_and_breadth() {
        let hotp = Hotp::new("A strong shared secret".as_bytes().to_vec());
        let code = hotp.make(MakeOption::Counter(42));
        let check = hotp.check(
            code.as_str(),
            CheckOption::Full {
                counter: 42,
                breadth: 6,
                algorithm: DEFAULT_ALGORITHM,
            },
        );
        assert!(check);
    }

    #[test]
    fn check_u64_to_8_length_u8_array() {
        let value = 1024_u64;
        let result = u64_to_8_length_u8_array(value);
        let expected = [00_u8, 00_u8, 00_u8, 00_u8, 00_u8, 00_u8, 4_u8, 00_u8];
        assert_eq!(result, expected)
    }

    #[test]
    fn check_max_u64_to_8_length_u8_array() {
        let value = u64::MAX;
        let result = u64_to_8_length_u8_array(value);
        let expected = [
            255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8,
        ];
        assert_eq!(result, expected)
    }
}
