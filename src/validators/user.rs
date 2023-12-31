use crate::{models::user::UserModel, prelude::Validateble};
use regex::Regex;

impl Validateble for UserModel {
    type Validated = ValidatedUserModel;

    type ValidationError = UserValidationError;

    fn validate(self, _: &()) -> Result<Self::Validated, Vec<Self::ValidationError>> {
        let mut errors = Vec::new();
        let mut lambda = |err: Vec<UserValidationError>| {
            let mut err = err;
            errors.append(&mut err);
            err
        };
        let _ = validate_password(self.password()).map_err(|err| lambda(err));
        let _ = validate_name(self.first_name()).map_err(|err| lambda(err));
        let _ = validate_name(self.last_name()).map_err(|err| lambda(err));
        let _ = validate_email(self.email()).map_err(|err| lambda(err));
        let _ = validate_phone_number(self.phone_number()).map_err(|err| lambda(err));
        let _ = validate_username(self.username()).map_err(|err| lambda(err));
        if self.about().len() > 500 {
            errors.push(UserValidationError::InvalidAbout)
        }
        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(ValidatedUserModel(self))
    }

    type Target = ();
}

pub struct ValidatedUserModel(UserModel);

pub(super) fn validate_password(password: String) -> Result<(), Vec<UserValidationError>> {
    let mut errors = Vec::new();
    if password.len() < 8 {
        errors.push(UserValidationError::InvalidPassword(
            InvalidPassword::PasswordToLong,
        ))
    }

    if password.len() > 24 {
        errors.push(UserValidationError::InvalidPassword(
            InvalidPassword::PasswordToLong,
        ))
    }

    if errors.is_empty() {
        return Ok(());
    }
    Err(errors)
}

pub(super) fn validate_email(email: String) -> Result<(), Vec<UserValidationError>> {
    let regex = Regex::new(r#"^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$"#).unwrap();
    if regex.is_match(&email) {
        return Ok(());
    }
    Err(vec![UserValidationError::InvalidEmail])
}

pub(super) fn validate_phone_number(
    phone_number: Option<String>,
) -> Result<(), Vec<UserValidationError>> {
    let mut errors = Vec::new();
    if phone_number.is_none() {
        return Ok(());
    }
    let phone_number = phone_number.unwrap();
    match phone_number.len().partial_cmp(&12) {
        Some(cmp) => match cmp {
            std::cmp::Ordering::Less => errors.push(UserValidationError::InvalidPhoneNumber(
                InvalidPhoneNumber::PhoneNumberToSmall,
            )),
            std::cmp::Ordering::Equal => return Ok(()),
            std::cmp::Ordering::Greater => errors.push(UserValidationError::InvalidPhoneNumber(
                InvalidPhoneNumber::PhoneNumberToLong,
            )),
        },
        None => errors.push(UserValidationError::InvalidPhoneNumber(
            InvalidPhoneNumber::PhoneNumberToSmall,
        )),
    };
    if !(phone_number.starts_with("998") && phone_number.parse::<u64>().is_ok()) {
        errors.push(UserValidationError::InvalidPhoneNumber(
            InvalidPhoneNumber::PhoneNumberContainsOtherLiterals,
        ))
    }
    if errors.is_empty() {
        return Ok(());
    }
    Err(errors)
}

pub(super) fn validate_username(username: String) -> Result<(), Vec<UserValidationError>> {
    let mut errors = Vec::new();

    if !username.is_ascii() {
        errors.push(UserValidationError::InvalidUsername(
            InvalidUsername::NotAscii,
        ))
    }

    Ok(())
}

pub(super) fn validate_name(name: String) -> Result<(), Vec<UserValidationError>> {
    if name.len() > 31 {
        return Err(vec![UserValidationError::InvalidName]);
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub enum UserValidationError {
    InvalidPhoneNumber(InvalidPhoneNumber),
    InvalidPassword(InvalidPassword),
    InvalidName,
    InvalidUsername(InvalidUsername),
    InvalidAbout,
    InvalidEmail,
}

#[derive(Debug, Clone)]
pub enum InvalidPhoneNumber {
    PhoneNumberToLong,
    PhoneNumberToSmall,
    PhoneNumberContainsOtherLiterals,
}

#[derive(Debug, Clone)]
pub enum InvalidPassword {
    PasswordToLong,
    PasswordToShort,
}

#[derive(Debug, Clone)]
pub enum InvalidUsername {
    NotAscii,
    ToSmall,
    ToLarge,
}
