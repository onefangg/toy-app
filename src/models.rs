use serde::{Deserialize};
use crate::errors::{PasswordError, UsernameError};

#[derive(Deserialize)]
#[serde(try_from = "String")]
pub struct Password(String);

impl Password {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Password {
    type Error = PasswordError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < 8 {
            return Err(PasswordError::TooShort);
        }
        Ok(Password(value))
    }
}

#[derive(Deserialize)]
#[serde(try_from = "String")]
pub struct Username(String);

impl Username {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Username {
    type Error = UsernameError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < 8 {
            return Err(UsernameError::TooShort);
        }
        Ok(Username(value))
    }
}

#[derive(serde::Deserialize)]
pub struct UserCredentialsForm {
    pub username: Username,
    pub password: Password,
}
