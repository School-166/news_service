use super::{controlller::Class, Subject, UserController, UserModel, UserRepo, UserType};
use crate::{get_db_pool, handler::users};
use actix_web::web::to;
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
}

async fn create_user(user_dto: UserModel, pool: &PgPool) -> Result<(), RegistrationError> {
    if !UserRepo::get_instance()
        .await
        .is_username_free(user_dto.username())
        .await
    {
        return Err(RegistrationError::UsernameAlreadyExists);
    }

    let user_type = match user_dto.user_specs() {
        UserType::Teacher {
            subject: _,
            school: _,
        } => "Teacher",
        UserType::Pupil {
            class: _,
            school: _,
        } => "Student",
        UserType::Administrator {
            job_title: _,
            school: _,
        } => "Administrator",
        UserType::Other => "Other",
    };

    let sql = "insert into users 
                                    (username, password, email, first_name, last_name, phone_number, user_type)
                             values ($1, $2, $3, $4, $5, $6, $7);";
    if sqlx::query(sql)
        .bind(user_dto.username())
        .bind(user_dto.password())
        .bind(user_dto.email())
        .bind(user_dto.first_name())
        .bind(user_dto.last_name())
        .bind(user_dto.phone_number())
        .bind(user_type)
        .execute(pool)
        .await
        .is_err()
    {
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
        if self.is_username_free(user_dto.username()).await {
            return Err(RegistrationError::UsernameAlreadyExists);
        }

        todo!()
    }

    pub fn get_by(&self, params: Vec<GetByQuery>) -> Option<UserController> {
        todo!()
    }

    pub async fn is_username_free(&self, username: String) -> bool {
        self.get_by(vec![GetByQuery::Username(username)]).is_none()
    }

    pub async fn change_params(&self, params: Vec<ChangeParamQuery>, model: UserModel) -> () {
        for param in params {
            match param {
                ChangeParamQuery::Password(_) => todo!(),
                ChangeParamQuery::Email(_) => todo!(),
                ChangeParamQuery::PhoneNumber(_) => todo!(),
                ChangeParamQuery::FirstName(_) => todo!(),
                ChangeParamQuery::LastName(_) => todo!(),
                ChangeParamQuery::ClassNumber(_) => todo!(),
                ChangeParamQuery::ClassChar(_) => todo!(),
                ChangeParamQuery::Class(_) => {
                    if !model.user_specs().is_pupil() {
                        return;
                    }
                }
            }
        }
    }
}

pub enum ChangeParamQuery {
    Password(String),
    Email(Option<String>),
    PhoneNumber(Option<String>),
    FirstName(String),
    LastName(String),
    ClassNumber(u8),
    ClassChar(u8),
    Class(Class),
}

pub enum GetByQuery {
    Username(String),
    LastName(String),
    FirstName(String),
    Email(String),
    PhoneNumber(String),
    UserSpecs(UserType),
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
            user_specs: match user_specs {
                "Teacher" => UserType::Teacher {
                    subject: Subject::from_str(row.get("subject")).unwrap(),
                    school: row.get("school"),
                },
                "Student" => UserType::Pupil {
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
                    school: row.get("school"),
                },
                "Administrator" => UserType::Administrator {
                    job_title: row.get("job_title"),
                    school: row.get("school"),
                },
                "Other" => UserType::Other,
                _ => panic!("Enum can't contain this value"),
            },
        })
    }
}
