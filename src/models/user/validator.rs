use super::repository::{ChangeParamQuery, UserTableParams};

fn validate_change_query(query: &ChangeParamQuery) -> Result<(), Vec<ChangeQueryValidationError>> {
    let mut errors = Vec::new();

    match query {
        ChangeParamQuery::UserTable(user_table) => match user_table {
            UserTableParams::Password(_) => todo!(),
            UserTableParams::Email(_) => todo!(),
            UserTableParams::PhoneNumber(phone_number) => {
                if phone_number.is_none() {
                    return Ok(());
                }
                let phone_number = phone_number.unwrap();

                match phone_number.len().partial_cmp(&12) {
                    Some(cmp) => match cmp {
                        std::cmp::Ordering::Less => {
                            errors.push(ChangeQueryValidationError::InvalidPhoneNumber(
                                InvalidNumber::PhoneNumberToSmall,
                            ))
                        }
                        std::cmp::Ordering::Equal => return Ok(()),
                        std::cmp::Ordering::Greater => {
                            errors.push(ChangeQueryValidationError::InvalidPhoneNumber(
                                InvalidNumber::PhoneNumberToLong,
                            ))
                        }
                    },
                    None => errors.push(ChangeQueryValidationError::InvalidPhoneNumber(
                        InvalidNumber::PhoneNumberToSmall,
                    )),
                };
            }
            UserTableParams::FirstName(_) => todo!(),
            UserTableParams::LastName(_) => todo!(),
        },
        ChangeParamQuery::JobTitle(_) => todo!(),
        ChangeParamQuery::Class(_) => todo!(),
    }
    Err(errors)
}

pub struct ValidatedChangeParamQuery(ChangeParamQuery);

impl ValidatedChangeParamQuery {
    pub fn parametr(&self) -> ChangeParamQuery {
        self.0
    }
}

pub enum ChangeQueryValidationError {
    InvalidPhoneNumber(InvalidNumber),
}

pub enum InvalidNumber {
    PhoneNumberToSmall,
    PhoneNumberToLong,
    PhoneNumbersContainsNotDigits,
}
