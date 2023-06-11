use nutype::nutype;

use super::response::{Rena, ToRena};

#[nutype(sanitize(trim) validate(min_len=4, max_len=16))]
pub struct Username(String);

impl<'a> ToRena<'a, ()> for UsernameError {
    fn to_rena(self, (..): ()) -> Rena<'a> {
        let response = match self {
            UsernameError::TooShort => "Username is too short",
            UsernameError::TooLong => "Username is too long",
        };

        Rena::no(self, vec![response])
    }
}

#[nutype(sanitize(trim))]
pub struct Email(String);

#[nutype(validate(min_len = 8, max_len = 32))]
pub struct Password(String);

impl<'a> ToRena<'a, ()> for PasswordError {
    fn to_rena(self, (..): ()) -> Rena<'a> {
        let response = match self {
            PasswordError::TooShort => "Password must have atleast 8 characters",
            PasswordError::TooLong => "Password can't have more than 32 characters",
        };

        Rena::no(self, vec![response])
    }
}
