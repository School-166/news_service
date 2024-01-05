use serde::Deserialize;
use uuid::Uuid;

use crate::{
    models::user::{UserModel, UserType},
    prelude::ToSQL,
    types::Class,
    validators::repository_query::users::{ValidatedChangeQueryParam, ValidationError},
};

trait GetParametr: ToSQL {}

pub struct UuidParametr(Uuid);
impl UuidParametr {
    pub fn new(uuid: &Uuid) -> UuidParametr {
        UuidParametr(uuid.clone())
    }
}
impl ToSQL for UuidParametr {
    fn to_sql(&self) -> String {
        format!("uuid = '{}'", self.0.to_string())
    }
}
impl GetParametr for UuidParametr {}

pub struct UsernameParametr(String);
impl UsernameParametr {
    pub fn new(username: &str) -> UsernameParametr {
        UsernameParametr(username.to_string())
    }
}
impl ToSQL for UsernameParametr {
    fn to_sql(&self) -> String {
        format!("username = '{}'", self.0.clone())
    }
}
impl GetParametr for UsernameParametr {}

pub struct LastNameParametr(String);
impl LastNameParametr {
    pub fn new(last_name: &str) -> LastNameParametr {
        LastNameParametr(last_name.to_string())
    }
}

pub enum GetByQueryParam {
    Uuid(Uuid),
    Username(String),
    LastName(String),
    FirstName(String),
    Email(String),
    PhoneNumber(String),
    UserSpecs(UserType),
}

impl ToSQL for GetByQueryParam {
    fn to_sql(&self) -> String {
        match self {
            GetByQueryParam::Uuid(uuid) => format!("users.uuid = '{}'", uuid),
            GetByQueryParam::Username(username) => format!("users.username = '{}'", username),
            GetByQueryParam::LastName(last_name) => format!("users.last_name = '{}'", last_name),
            GetByQueryParam::FirstName(first_name) => {
                format!("users.first_name = '{}'", first_name)
            }
            GetByQueryParam::Email(email) => format!("users.email = '{}'", email),
            GetByQueryParam::PhoneNumber(phone_number) => {
                format!("users.phone_number = '{}'", phone_number)
            }
            GetByQueryParam::UserSpecs(specs) => format!(
                "users.user_specs = '{}'",
                match specs {
                    UserType::Teacher { subject: _ } => "Teacher",
                    UserType::Student { class: _ } => "Student",
                    UserType::Administrator { job_title: _ } => "Administrator",
                    UserType::Other => "Other",
                }
            ),
        }
    }
}

#[derive(Debug)]
pub enum ChangeParamsError {
    UserDoesntExist,
    ClassParametrChangingForNotStudent,
    ChangingJobTitleForNotAdministrator,
    ValidationError(Vec<ValidationError>),
    DBProblems,
}
impl ChangeQueryParam {
    fn select_table(&self) -> String {
        match self {
            ChangeQueryParam::JobTitle(_) => "administrators",
            ChangeQueryParam::Class(_) => "students",
            _ => "users",
        }
        .to_string()
    }
}

impl ToSQL for ChangeQuery {
    fn to_sql(&self) -> String {
        format!(
            "update {} where username= '{}' set {};",
            self.param.param().select_table(),
            self.target_username,
            self.param.param().to_sql()
        )
    }
}

impl ToSQL for ChangeQueryParam {
    fn to_sql(&self) -> String {
        format!(
            "{}.{}",
            self.select_table(),
            match self {
                ChangeQueryParam::Password(password) => format!("password = '{}'", password),
                ChangeQueryParam::Email(email) => format!("email = '{}'", email),
                ChangeQueryParam::PhoneNumber(phone_num) => format!(
                    "phone_number = {}",
                    phone_num
                        .clone()
                        .map_or("NULL".to_string(), |phone_num| format!("'{}'", phone_num))
                ),
                ChangeQueryParam::FirstName(first_name) => format!("first_name = '{}'", first_name),
                ChangeQueryParam::LastName(last_name) => format!("last_name = '{}'", last_name),
                ChangeQueryParam::JobTitle(job_title) => format!("job_title = '{}'", job_title),
                ChangeQueryParam::Class(class) => format!(
                    "class_num = {}, class_char = '{}'",
                    class.class_num(),
                    class.class_char()
                ),
                ChangeQueryParam::About(about) => format!("about = '{}'", about),
            }
        )
    }
}

pub struct ChangeQuery {
    target_username: String,
    param: ValidatedChangeQueryParam,
}

impl ChangeQuery {
    pub fn new(target: &UserModel, param: ValidatedChangeQueryParam) -> Self {
        Self {
            target_username: target.username(),
            param,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum ChangeQueryParam {
    Password(String),
    About(String),
    Email(String),
    PhoneNumber(Option<String>),
    FirstName(String),
    LastName(String),
    JobTitle(String),
    Class(Class),
}
