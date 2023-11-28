use super::{
    controller::Class, validator::ValidatedChangeParamQuery, Subject, UserController, UserModel,
    UserRepo, UserType,
};
use crate::{get_db_pool, models::Model};
use async_once::AsyncOnce;
use core::panic;
use lazy_static::lazy_static;
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::str::FromStr;

lazy_static! {
    static ref USER_REPO: AsyncOnce<UserRepo> =
        AsyncOnce::new(async { UserRepo(get_db_pool().await) });
}

pub enum RegistrationError {
    UsernameAlreadyExists,
    ProblemsWithDB,
    ErrorsOnRegisttrationUserType,
}

async fn create_user(user_dto: &UserModel, pool: &PgPool) -> Result<(), RegistrationError> {
    if !UserRepo::get_instance()
        .await
        .is_username_free(user_dto.username())
        .await
    {
        return Err(RegistrationError::UsernameAlreadyExists);
    }

    let user_type = match user_dto.user_specs() {
        UserType::Teacher { subject: _ } => "Teacher",
        UserType::Student { class: _ } => "Student",
        UserType::Administrator { job_title: _ } => "Administrator",
        UserType::Other => "Other",
    };

    let sql = "insert into users 
                       (username, password, email, first_name, last_name, phone_number, user_type, birth_date)
                    values ($1, $2, $3, $4, $5, $6, $7, $8);";
    if sqlx::query(sql)
        .bind(user_dto.username())
        .bind(user_dto.password())
        .bind(user_dto.email())
        .bind(user_dto.first_name())
        .bind(user_dto.last_name())
        .bind(user_dto.phone_number())
        .bind(user_type)
        .bind(user_dto.birth_date())
        .execute(pool)
        .await
        .is_err()
    {
        return Err(RegistrationError::ProblemsWithDB);
    }
    Ok(())
}

async fn create_user_specs(user_dto: &UserModel, pool: &PgPool) -> Result<(), RegistrationError> {
    let sql = format!(
        "insert into {};",
        match user_dto.user_specs() {
            UserType::Teacher { subject: _ } => "teachers (username, subject) values ($1, $2)",
            UserType::Student { class: _ } =>
                "students (username, class_num, class_char) values ($1, $2, $3)",
            UserType::Administrator { job_title: _ } =>
                "administrators (username, job_title) values($1, $2)",
            UserType::Other => return Ok(()),
        }
    );
    let mut query = sqlx::query(&sql).bind(user_dto.username());
    query = match user_dto.user_specs() {
        UserType::Teacher { subject } => query.bind(subject.to_string()),
        UserType::Student { class } => query.bind(class.class_num() as i8).bind(class.class_char()),
        UserType::Administrator { job_title } => query.bind(job_title),
        UserType::Other => return Ok(()),
    };

    if query.execute(pool).await.is_err() {
        return Err(RegistrationError::ProblemsWithDB);
    }
    Ok(())
}

impl UserRepo {
    pub async fn get_instance() -> &'static Self {
        USER_REPO.get().await
    }

    pub async fn register_user(
        &self,
        user_dto: UserModel,
    ) -> Result<UserController, RegistrationError> {
        create_user(&user_dto, self.0).await?;
        create_user_specs(&user_dto, self.0).await?;
        Ok(self
            .get_one_by(vec![GetByQuery::Username(user_dto.username())])
            .await
            .expect("unreacheble"))
    }

    pub async fn get_one_by(&self, params: Vec<GetByQuery>) -> Option<UserController> {
        let sql = match build_get_by_sql(params) {
            Ok(value) => value,
            Err(_) => return None,
        };
        match sqlx::query_as::<_, UserModel>(&sql).fetch_one(self.0).await {
            Ok(user) => Some(user.controller()),
            Err(_) => None,
        }
    }

    pub async fn get_many_by(&self, params: Vec<GetByQuery>) -> Vec<UserController> {
        let sql = match build_get_by_sql(params) {
            Ok(value) => value,
            Err(_) => return Vec::new(),
        };
        match sqlx::query_as::<_, UserModel>(&sql).fetch_all(self.0).await {
            Ok(users) => users.iter().map(|user| user.controller()).collect(),
            Err(_) => Vec::new(),
        }
    }

    pub async fn is_username_free(&self, username: String) -> bool {
        self.get_one_by(vec![GetByQuery::Username(username)])
            .await
            .is_none()
    }

    pub async fn change_params(
        &self,
        params: Vec<ValidatedChangeParamQuery>,
        model: UserModel,
    ) -> Result<(), ChangeParamsError> {
        if self.is_username_free(model.username()).await {
            return Err(ChangeParamsError::UserDoesntExist);
        }

        for param in params {
            bind_change_param_query(param.parametr(), &model, self.0).await?;
        }
        Ok(())
    }
}

fn build_get_by_sql(params: Vec<GetByQuery>) -> Result<String, ()> {
    let conditions = {
        let first_conditon = params.first();
        if first_conditon.is_none() {
            return Err(());
        }
        let first_condition = first_conditon.unwrap();
        let mut conditions = first_condition.to_sql();
        for param in params.iter().next() {
            conditions = format!("{} and {}", conditions, param.to_sql())
        }
        conditions
    };
    let sql = format!(
        "select * from users 
            join pupils on users.username = pupils.username 
            join administrators on users.username = administrators.username 
            join teachers on users.username = teachers.username where {};",
        conditions
    );
    Ok(sql)
}

#[derive(Clone)]
pub enum ChangeParamQuery {
    UserTable(UserTableParams),
    JobTitle(String),
    Class(Class),
}

async fn bind_change_param_query(
    param: ChangeParamQuery,
    model: &UserModel,
    pool: &PgPool,
) -> Result<(), ChangeParamsError> {
    let sql = build_changing_param_sql(choose_table(param.clone(), model)?, param.clone());
    let mut query = sqlx::query(&sql).bind(model.username());
    query = match param.clone() {
        ChangeParamQuery::UserTable(param) => match param {
            UserTableParams::Password(password) => query.bind(password),
            UserTableParams::Email(email) => query.bind(email),
            UserTableParams::PhoneNumber(phone_number) => query.bind(phone_number),
            UserTableParams::FirstName(first_name) => query.bind(first_name),
            UserTableParams::LastName(last_name) => query.bind(last_name),
        },
        ChangeParamQuery::JobTitle(job_title) => query.bind(job_title),
        ChangeParamQuery::Class(class) => {
            query.bind(class.class_num() as i8).bind(class.class_char())
        }
    };
    if query.execute(pool).await.is_err() {
        return Err(ChangeParamsError::DBProblems);
    }
    Ok(())
}

fn choose_table(param: ChangeParamQuery, model: &UserModel) -> Result<String, ChangeParamsError> {
    let table = match param {
        ChangeParamQuery::UserTable(_) => "users",
        ChangeParamQuery::JobTitle(_) => {
            if !model.user_specs().is_administrator() {
                return Err(ChangeParamsError::ChangingJobTitleForNotAdministrator);
            }
            "administrators"
        }
        ChangeParamQuery::Class(_) => {
            if !model.user_specs().is_pupil() {
                return Err(ChangeParamsError::ClassParametrChangingForNotStudent);
            }
            "students"
        }
    }
    .to_string();
    Ok(table)
}

fn build_changing_param_sql(table: String, param: ChangeParamQuery) -> String {
    format!(
        "update {} where username=$1 set {};",
        table,
        match param {
            ChangeParamQuery::UserTable(param) => match param {
                UserTableParams::Password(_) => "password = $2",
                UserTableParams::Email(_) => "email = $2",
                UserTableParams::PhoneNumber(_) => "phone_number = $2",
                UserTableParams::FirstName(_) => "first_name = $2",
                UserTableParams::LastName(_) => "last_name = $2",
            },
            ChangeParamQuery::JobTitle(_) => "job_title = $2",
            ChangeParamQuery::Class(_) => "class_num = $2, class_char = $3",
        }
    )
}

#[derive(Clone)]
pub enum UserTableParams {
    Password(String),
    Email(String),
    PhoneNumber(Option<String>),
    FirstName(String),
    LastName(String),
}

pub enum GetByQuery {
    Username(String),
    LastName(String),
    FirstName(String),
    Email(String),
    PhoneNumber(String),
    UserSpecs(UserType),
}

impl GetByQuery {
    pub fn to_sql(&self) -> String {
        match self {
            GetByQuery::Username(username) => format!("users.username = '{}'", username),
            GetByQuery::LastName(last_name) => format!("users.last_name = '{}'", last_name),
            GetByQuery::FirstName(first_name) => format!("users.first_name = '{}'", first_name),
            GetByQuery::Email(email) => format!("users.email = '{}'", email),
            GetByQuery::PhoneNumber(phone_number) => {
                format!("users.phone_number = '{}'", phone_number)
            }
            GetByQuery::UserSpecs(specs) => format!(
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

impl FromRow<'_, PgRow> for UserModel {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let user_specs: &str = row.get("user_specs");
        Ok(UserModel {
            username: row.get("username"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            password: row.get("password"),
            email: row.get("email"),
            phone_number: row.get("phone_number"),
            birth_date: row.get("birth_date"),
            about: row.get("about"),
            user_specs: match user_specs {
                "Teacher" => UserType::Teacher {
                    subject: Subject::from_str(row.get("subject")).unwrap(),
                },
                "Student" => UserType::Student {
                    class: Class::from(
                        *{
                            let class_char: String = row.get("class_char");
                            class_char
                        }
                        .as_bytes()
                        .first()
                        .unwrap(),
                        *{
                            let class_num: String = row.get("class_number");
                            class_num
                        }
                        .as_bytes()
                        .first()
                        .unwrap(),
                    )
                    .unwrap(),
                },
                "Administrator" => UserType::Administrator {
                    job_title: row.get("job_title"),
                },
                "Other" => UserType::Other,
                _ => panic!("Enum can't contain this value"),
            },
        })
    }
}
