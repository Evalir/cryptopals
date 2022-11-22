// Set 1
fn challenge_1(s: &str) -> String {
    let bytes = hex::decode(s).unwrap();
    base64::encode(bytes)
}

fn challenge_2(s1: &str, s2: &str) -> String {
    let bytes = hex::decode(s1).unwrap();
    let input_bytes = hex::decode(s2).unwrap();
    assert!(bytes.len() == input_bytes.len());

    let ans: Vec<_> = bytes.iter().zip(input_bytes).map(|(x, y)| x ^ y).collect();
    hex::encode(ans)
}

// See https://cryptopals.com/sets/1/challenges/3
fn challenge_3(s: &str) -> String {
    todo!()
}

fn main() {
    println!("{}", &challenge_1("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
}

#[cfg(test)]
mod tests {
    use crate::{challenge_1, challenge_2};

    #[test]
    pub fn challenge_1_ok() {
        let test_cases = vec![("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d", "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")];
        for (input, output) in test_cases {
            assert_eq!(challenge_1(input), output, "The output did not match");
        }
    }

    #[test]
    pub fn challenge_2_ok() {
        let test_cases = vec![(
            "1c0111001f010100061a024b53535009181c",
            "686974207468652062756c6c277320657965",
            "746865206b696420646f6e277420706c6179",
        )];
        for (s1, s2, expected) in test_cases {
            let actual = challenge_2(s1, s2);
            assert_eq!(
                actual, expected,
                "The output did not match: {} vs {}",
                actual, expected
            );
        }
    }
}
