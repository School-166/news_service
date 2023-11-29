use self::queries::{
    validation::ValidatedChangeQueryParam, ChangeParamsError, ChangeQuery, GetByQuery,
    GetByQueryParam, ToSql,
};
use super::{controller::Class, Subject, UserController, UserModel, UserRepo, UserType};
use crate::{get_db_pool, models::Model};
use async_once::AsyncOnce;
use core::panic;
use lazy_static::lazy_static;
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::str::FromStr;

pub mod queries;

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
                       (username, password, email, first_name, last_name, phone_number, user_type, birth_date, about)
                    values ($1, $2, $3, $4, $5, $6, $7, $8, $9);";
    if sqlx::query(sql)
        .bind(user_dto.username())
        .bind(user_dto.password())
        .bind(user_dto.email())
        .bind(user_dto.first_name())
        .bind(user_dto.last_name())
        .bind(user_dto.phone_number())
        .bind(user_type)
        .bind(user_dto.birth_date())
        .bind(user_dto.about())
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
            .get_one_by(vec![GetByQueryParam::Username(user_dto.username())])
            .await
            .expect("unreacheble"))
    }

    pub async fn get_one_by(&self, params: Vec<GetByQueryParam>) -> Option<UserController> {
        match sqlx::query_as::<_, UserModel>(&GetByQuery::new(params).to_sql())
            .fetch_one(self.0)
            .await
        {
            Ok(user) => Some(user.controller()),
            Err(_) => None,
        }
    }

    pub async fn get_many_by(&self, params: Vec<GetByQueryParam>) -> Vec<UserController> {
        match sqlx::query_as::<_, UserModel>(&GetByQuery::new(params).to_sql())
            .fetch_all(self.0)
            .await
        {
            Ok(users) => users.iter().map(|user| user.controller()).collect(),
            Err(_) => Vec::new(),
        }
    }

    pub async fn is_username_free(&self, username: String) -> bool {
        self.get_one_by(vec![GetByQueryParam::Username(username)])
            .await
            .is_none()
    }

    pub async fn change_params(
        &self,
        params: Vec<ValidatedChangeQueryParam>,
        model: UserModel,
    ) -> Result<(), ChangeParamsError> {
        if self.is_username_free(model.username()).await {
            return Err(ChangeParamsError::UserDoesntExist);
        }

        for param in params {
            sqlx::query(&ChangeQuery::new(&model, param).to_sql())
                .execute(self.0)
                .await;
        }
        Ok(())
    }
}

impl FromRow<'_, PgRow> for UserType {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let user_specs: &str = row.get("user_specs");
        Ok(match user_specs {
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
        })
    }
}

impl FromRow<'_, PgRow> for UserModel {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(UserModel {
            username: row.get("username"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            password: row.get("password"),
            email: row.get("email"),
            phone_number: row.get("phone_number"),
            birth_date: row.get("birth_date"),
            about: row.get("about"),
            user_specs: UserType::from_row(row)?,
        })
    }
}
