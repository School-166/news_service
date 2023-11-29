use crate::models::user::{controller::Class, UserModel, UserType};

use self::validation::ValidatedChangeQueryParam;
pub mod validation;

pub enum GetByQueryParam {
    Username(String),
    LastName(String),
    FirstName(String),
    Email(String),
    PhoneNumber(String),
    UserSpecs(UserType),
}

pub struct GetByQuery {
    params: Vec<GetByQueryParam>,
}

impl ToSql for GetByQuery {
    fn to_sql(&self) -> String {
        let conditions = {
            let first_conditon = self.params.first();
            if first_conditon.is_none() {
                "users.username = NULL".to_string()
            } else {
                let first_condition = first_conditon.unwrap();
                let mut conditions = first_condition.to_sql();
                for param in self.params.iter().next() {
                    conditions = format!("{} and {}", conditions, param.to_sql())
                }
                conditions
            }
        };
        format!(
            "select * from users 
            join pupils on users.username = pupils.username 
            join administrators on users.username = administrators.username 
            join teachers on users.username = teachers.username where {};",
            conditions
        )
    }
}

impl GetByQuery {
    pub fn new(params: Vec<GetByQueryParam>) -> Self {
        Self { params }
    }
}

pub trait ToSql {
    fn to_sql(&self) -> String;
}

pub trait Validateble {
    type Validated;
    type ValidationError;
    fn validate(self, target: &UserModel) -> Result<Self::Validated, Vec<Self::ValidationError>>;
}

impl ToSql for GetByQueryParam {
    fn to_sql(&self) -> String {
        match self {
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
                    UserType::Teacher { subject } => "Teacher",
                    UserType::Student { class } => "Student",
                    UserType::Administrator { job_title } => "Administrator",
                    UserType::Other => "Other",
                }
            ),
        }
    }
}

pub enum ChangeParamsError {
    UserDoesntExist,
    ClassParametrChangingForNotStudent,
    ChangingJobTitleForNotAdministrator,
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

impl ToSql for ChangeQuery {
    fn to_sql(&self) -> String {
        format!(
            "update {} where username= '{}' set {};",
            self.param.param().select_table(),
            self.target_username,
            self.param.param().to_sql()
        )
    }
}

impl ToSql for ChangeQueryParam {
    fn to_sql(&self) -> String {
        format!(
            "{}.{}",
            self.select_table(),
            match self {
                ChangeQueryParam::Password(password) => format!("password = '{}'", password),
                ChangeQueryParam::Email(email) => format!("email = '{}'", email),
                ChangeQueryParam::PhoneNumber(phone_num) => format!(
                    "phone_number = {}",
                    phone_num.map_or("NULL".to_string(), |phone_num| format!("'{}'", phone_num))
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

#[derive(Clone)]
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
