use regex::Regex;

use crate::models::user::UserModel;

use super::{ChangeQueryParam, Validateble};

pub struct ValidatedChangeQueryParam(ChangeQueryParam);

impl Validateble for ChangeQueryParam {
    type Validated = ValidatedChangeQueryParam;

    type ValidationError = ValidationError;

    fn validate(self, target: &UserModel) -> Result<Self::Validated, Vec<Self::ValidationError>> {
        ValidatedChangeQueryParam::validate(self, target)
    }
}

impl ValidatedChangeQueryParam {
    fn validate(param: ChangeQueryParam, target: &UserModel) -> Result<Self, Vec<ValidationError>> {
        match param.clone() {
            ChangeQueryParam::Password(password) => validate_password(password)?,
            ChangeQueryParam::Email(email) => validate_email(email)?,
            ChangeQueryParam::PhoneNumber(phone_num) => validate_phone_number(phone_num)?,
            ChangeQueryParam::FirstName(first_name) => validate_name(first_name)?,
            ChangeQueryParam::LastName(last_name) => validate_name(last_name)?,
            ChangeQueryParam::About(about) => {
                if about.len() > 500 {
                    return Err(vec![ValidationError::InvalidAbout]);
                }
            }
            ChangeQueryParam::JobTitle(job_title) => {
                if job_title.len() > 31 {
                    return Err(vec![ValidationError::InvalidJobTitle]);
                }
                if !target.user_specs().is_administrator() {
                    return Err(vec![ValidationError::ChangingJobTitleForNotAdministrator]);
                }
            }
            ChangeQueryParam::Class(_) => {
                if !target.user_specs().is_student() {
                    return Err(vec![ValidationError::ChangingClassForNotStudent]);
                }
            }
        }
        Ok(ValidatedChangeQueryParam(param))
    }

    pub fn param(&self) -> ChangeQueryParam {
        self.0.clone()
    }
}

fn validate_password(password: String) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    if password.len() < 8 {
        errors.push(ValidationError::InvalidPassword(
            InvalidPassword::PasswordToLong,
        ))
    }

    if password.len() > 24 {
        errors.push(ValidationError::InvalidPassword(
            InvalidPassword::PasswordToLong,
        ))
    }

    if errors.is_empty() {
        return Ok(());
    }
    Err(errors)
}

fn validate_email(email: String) -> Result<(), Vec<ValidationError>> {
    let regex = Regex::new(r#"^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$"#).unwrap();
    if regex.is_match(&email) {
        return Ok(());
    }
    Err(vec![ValidationError::InvalidEmail])
}

fn validate_phone_number(phone_number: Option<String>) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    if phone_number.is_none() {
        return Ok(());
    }
    let phone_number = phone_number.unwrap();
    match phone_number.len().partial_cmp(&12) {
        Some(cmp) => match cmp {
            std::cmp::Ordering::Less => errors.push(ValidationError::InvalidPhoneNumber(
                InvalidPhoneNumber::PhoneNumberToSmall,
            )),
            std::cmp::Ordering::Equal => return Ok(()),
            std::cmp::Ordering::Greater => errors.push(ValidationError::InvalidPhoneNumber(
                InvalidPhoneNumber::PhoneNumberToLong,
            )),
        },
        None => errors.push(ValidationError::InvalidPhoneNumber(
            InvalidPhoneNumber::PhoneNumberToSmall,
        )),
    };
    if !(phone_number.starts_with("998") && phone_number.parse::<u64>().is_ok()) {
        errors.push(ValidationError::InvalidPhoneNumber(
            InvalidPhoneNumber::PhoneNumberContainsOtherLiterals,
        ))
    }
    if errors.is_empty() {
        return Ok(());
    }
    Err(errors)
}

fn validate_name(name: String) -> Result<(), Vec<ValidationError>> {
    if name.len() > 31 {
        return Err(vec![ValidationError::InvalidName]);
    }
    Ok(())
}

#[derive(Debug)]
pub enum ValidationError {
    InvalidPhoneNumber(InvalidPhoneNumber),
    InvalidPassword(InvalidPassword),
    InvalidName,
    InvalidAbout,
    InvalidEmail,
    InvalidJobTitle,
    ChangingJobTitleForNotAdministrator,
    ChangingClassForNotStudent,
}

#[derive(Debug)]
pub enum InvalidPhoneNumber {
    PhoneNumberToLong,
    PhoneNumberToSmall,
    PhoneNumberContainsOtherLiterals,
}

#[derive(Debug)]
pub enum InvalidPassword {
    PasswordToLong,
    PasswordToShort,
}
