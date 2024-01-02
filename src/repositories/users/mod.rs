use self::queries::{ChangeQuery, GetByQueryParam};
use crate::{
    dto::UserRegistrationDTO,
    get_db_pool,
    models::user::{UserModel, UserType},
    prelude::ToSQL,
    types::{Class, Subject},
    utils::sql::SelectRequestBuilder,
    validators::repository_query::users::ValidatedChangeQueryParam,
};
use serde::Serialize;
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::str::FromStr;
use uuid::Uuid;

pub mod queries;

pub struct UserRepo(PgPool);

#[derive(Serialize)]
pub enum RegistrationError {
    UsernameAlreadyExists,
    ProblemsWithDB,
    ErrorsOnRegisttrationUserType,
}

async fn create_user(
    uuid: Uuid,
    user_dto: &UserRegistrationDTO,
    repo: &UserRepo,
) -> Result<(), RegistrationError> {
    if !repo.is_username_free(user_dto.username.clone()).await {
        return Err(RegistrationError::UsernameAlreadyExists);
    }

    let user_type = match user_dto.user_specs {
        UserType::Teacher { subject: _ } => UserTypeFromRow::Teacher,
        UserType::Student { class: _ } => UserTypeFromRow::Student,
        UserType::Administrator { job_title: _ } => UserTypeFromRow::Administrator,
        UserType::Other => UserTypeFromRow::Other,
    };

    let sql = format!("insert into users 
                       (uuid, username, password, email, first_name, last_name, phone_number, user_specs, birth_date, about)
                    values (\'{}\', $1, $2, $3, $4, $5, $6, $7, $8, $9);", uuid);
    sqlx::query(&sql)
        .bind(user_dto.username.clone())
        .bind(user_dto.password.clone())
        .bind(user_dto.email.clone())
        .bind(user_dto.first_name.clone())
        .bind(user_dto.last_name.clone())
        .bind(user_dto.phone_number.clone())
        .bind(user_type)
        .bind(user_dto.birth_date)
        .bind(user_dto.about.clone())
        .execute(&repo.pool())
        .await
        .expect("error in sql");
    Ok(())
}

async fn create_user_specs(
    user_dto: &UserRegistrationDTO,
    pool: &PgPool,
) -> Result<(), RegistrationError> {
    let username = user_dto.username.as_str();
    let sql = format!(
        "insert into {};",
        match user_dto.user_specs.clone() {
            UserType::Teacher { subject } => format!(
                "teachers (username, subject) values ('{}', '{}')",
                username,
                subject.to_string()
            ),
            UserType::Student { class } => format!(
                "students (username, class_num, class_char) values ('{}', {}, '{}')",
                username,
                class.class_num(),
                class.class_char()
            ),
            UserType::Administrator { job_title } => format!(
                "administrators (username, job_title) values('{}', '{}')",
                username, job_title
            ),
            UserType::Other => return Ok(()),
        }
    );
    let query = sqlx::query(&sql);

    if query.execute(pool).await.is_err() {
        return Err(RegistrationError::ProblemsWithDB);
    }
    Ok(())
}

impl UserRepo {
    pub async fn get_instance() -> Self {
        UserRepo(get_db_pool().await.clone())
    }

    fn pool(&self) -> PgPool {
        self.0.clone()
    }

    pub async fn get_by_uuid(&self, uuid: &Uuid) -> Option<UserModel> {
        self.get_one(vec![GetByQueryParam::Uuid(uuid.clone())])
            .await
    }

    pub async fn get_by_username(&self, username: &str) -> Option<UserModel> {
        self.get_one(vec![GetByQueryParam::Username(username.to_string())])
            .await
    }

    pub async fn register(
        &self,
        registration_dto: UserRegistrationDTO,
    ) -> Result<UserModel, RegistrationError> {
        let uuid = Uuid::new_v4();
        create_user(uuid.clone(), &registration_dto, &self).await?;
        create_user_specs(&registration_dto, &self.pool()).await?;
        Ok(self.get_by_uuid(&uuid).await.unwrap())
    }

    async fn get_one(&self, params: Vec<GetByQueryParam>) -> Option<UserModel> {
        self.get_many(params).await.first().map(|user| user.clone())
    }

    pub async fn get_many(&self, params: Vec<GetByQueryParam>) -> Vec<UserModel> {
        let sql = SelectRequestBuilder::<(), GetByQueryParam>::new(
            "select   
                 users.uuid,
                 users.username,
                 users.password,
                 users.email,
                 users.first_name,
                 users.last_name,
                 users.phone_number,
                 users.user_specs,
                 users.birth_date,
                 users.about,
                students.class_num,
                students.class_char,
                teachers.subject,
                administrators.job_title from users
            left outer join students on users.username = students.username 
            left outer join teachers on users.username = teachers.username 
            left outer join administrators on users.username = administrators.username"
                .to_string(),
            params,
        )
        .build();

        match sqlx::query_as::<_, UserModel>(&sql)
            .fetch_all(&self.pool())
            .await
        {
            Ok(users) => users,
            Err(_) => Vec::new(),
        }
    }

    pub async fn is_username_free(&self, username: String) -> bool {
        self.get_one(vec![GetByQueryParam::Username(username)])
            .await
            .is_none()
    }

    pub async fn change(&self, params: Vec<ValidatedChangeQueryParam>, model: UserModel) {
        for param in params {
            let _ = sqlx::query(&ChangeQuery::new(&model, param).to_sql())
                .execute(&self.pool())
                .await;
        }
    }
}

impl FromRow<'_, PgRow> for UserType {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let user_specs = row.get("user_specs");
        Ok(match user_specs {
            UserTypeFromRow::Teacher => UserType::Teacher {
                subject: Subject::from_str(row.get("subject")).unwrap(),
            },
            UserTypeFromRow::Student => UserType::Student {
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
            UserTypeFromRow::Administrator => UserType::Administrator {
                job_title: row.get("job_title"),
            },
            UserTypeFromRow::Other => UserType::Other,
        })
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "user_specs")]
enum UserTypeFromRow {
    Student,
    Administrator,
    Teacher,
    Other,
}
