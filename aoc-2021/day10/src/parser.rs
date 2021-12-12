use std::collections::HashMap;

pub const OPENINGS: [char; 4] = ['(', '[', '{', '<'];
pub const CLOSINGS: [char; 4] = [')', ']', '}', '>'];

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BError {
    Incomplete(Vec<char>),
    Corrupted(char),
    UnknownChar,
}

pub struct Validator<'a, 'b> {
    openings: &'a [char],
    closings: &'b [char],
    matches: HashMap<char, char>,
}

impl<'a, 'b> Validator<'a, 'b> {
    pub fn new(openings: &'a [char], closings: &'b [char]) -> Self {
        let mut hm = HashMap::new();
        for i in 0..openings.len() {
            hm.insert(closings[i], openings[i]);
        }

        Self {
            openings,
            closings,
            matches: hm,
        }
    }

    pub fn matches(&self) -> HashMap<char, char> {
        self.matches.clone()
    }

    pub fn is_valid(&self, s: &str) -> Result<(), BError> {
        // s.len() / 2 is a safe bet. If sequence is valid, stack won't rise above this size
        let mut stack = Vec::with_capacity(s.len() / 2);

        for c in s.chars() {
            // if a closing char
            if self.closings.contains(&c) {
                let mut does_match = || {
                    let top = stack.pop()?;
                    let matching = self.matches.get(&c)?;
                    Some(*matching == top)
                };

                if let Some(yes) = does_match() {
                    if yes {
                        // do nothing?
                    } else {
                        // closing does not match with opening
                        return Err(BError::Corrupted(c));
                    }
                } else {
                    return Err(BError::UnknownChar);
                }

            } else if self.openings.contains(&c) { // if an opening char
                stack.push(c);
            } else {
                // Unknown character encountered
                return Err(BError::UnknownChar);
            }
        }

        // Empty stack at the end is a sign that we have closed all the brackets
        if !stack.is_empty() {
            return Err(BError::Incomplete(stack));
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn usage_example() {
        let input = "()";
//        assert!(Validator::new(&OPENINGS, &CLOSINGS).is_valid(input));
    }

    macro_rules! validator_test {
        ($test_name:ident, $input:expr, $expected:expr) => {
            #[test]
            #[allow(clippy::bool_assert_comparison)]
            fn $test_name() {
                assert_eq!($expected, Validator::new(&OPENINGS, &CLOSINGS).is_valid($input));
            }
        };
    }

    validator_test!(sample1, "()", Ok(()));
    validator_test!(sample2, "()()", Ok(()));
    validator_test!(sample3, "(())", Ok(()));
    validator_test!(sample4, "()[]{}", Ok(()));
    validator_test!(sample5, "([{}])", Ok(()));
    validator_test!(sample6, "())", Err(BError::UnknownChar));
    // validator_test!(sample7, "(){", Err(BError::Incomplete));
    validator_test!(sample8, "{[}]", Err(BError::Corrupted('}')));
    // validator_test!(sample9, "[[[[[[]]]", Err(BError::Incomplete));
    validator_test!(sample10, "[)]", Err(BError::Corrupted(')')));
    validator_test!(sample11, "[(]", Err(BError::Corrupted(']')));
    // validator_test!(sample12, "[](", Err(BError::Incomplete));
    validator_test!(sample13, "(()())", Ok(()));
    validator_test!(unknown_char, "w", Err(BError::UnknownChar));

    // #[test]
    // fn rosetta_code() {
    //     let test_cases = vec![
    //         ("", true),
    //         ("[]", true),
    //         ("[]", true),
    //         ("[][]", true),
    //         ("[[][]]", true),
    //         ("][][", false),
    //         ("[]][[]", false),
    //     ];
    //     let validator = Validator::new(&OPENINGS, &CLOSINGS);

    //     for test_case in test_cases {
    //         assert_eq!(test_case.1, validator.is_valid(test_case.0));
    //     }
    // }
}
