use super::{controlller::Class, Subject, UserController, UserModel, UserRepo, UserType};
use crate::get_db_pool;
use actix_web::middleware::Condition;
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

async fn create_user_specs(user_dto: &UserModel, pool: &PgPool) -> Result<(), RegistrationError> {
    let sql = format!(
        "insert into {};",
        match user_dto.user_specs() {
            UserType::Teacher {
                subject: _,
                school: _,
            } => "teachers (username, subject, school) values ($1, $2, $3)",
            UserType::Pupil {
                class: _,
                school: _,
            } => "students (username, class_num, class_char, school) values ($1, $2, $3, $4)",
            UserType::Administrator {
                job_title: _,
                school: _,
            } => "administrators (username, job_title, school) values($1, $2, $3)",
            UserType::Other => return Ok(()),
        }
    );
    let mut query = sqlx::query(&sql).bind(user_dto.username());
    query = match user_dto.user_specs() {
        UserType::Teacher { subject, school } => query.bind(subject.to_string()).bind(school),
        UserType::Pupil { class, school } => query
            .bind(class.class_num() as i8)
            .bind(class.class_char())
            .bind(school),
        UserType::Administrator { job_title, school } => query.bind(job_title).bind(school),
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
            .get_by(vec![GetByQuery::Username(user_dto.username())])
            .expect("unreacheble"))
    }

    pub fn get_by(&self, params: Vec<GetByQuery>) -> Option<UserController> {
        
        let conditions = {
            let first_conditon = params.first();
            if first_conditon.is_none(){
                return None;
            } 
            let first_condition = first_conditon.unwrap();
            
            for i in 0..params.len(){match first_condition{
                GetByQuery::Username(_) => todo!(),
                GetByQuery::LastName(_) => todo!(),
                GetByQuery::FirstName(_) => todo!(),
                GetByQuery::Email(_) => todo!(),
                GetByQuery::PhoneNumber(_) => todo!(),
                GetByQuery::UserSpecs(_) => todo!(),
            }}
                   }
                
        
        let sql = format!("select * from users 
            join pupils on users.username = pupils.username 
            join administrators on users.username = administrators.username 
            join teachers on users.username = teachers.username where {};");
        todo!()
    }

    

    pub async fn is_username_free(&self, username: String) -> bool {
        self.get_by(vec![GetByQuery::Username(username)]).is_none()
    }

    pub async fn change_params(
        &self,
        params: Vec<ChangeParamQuery>,
        model: UserModel,
    ) -> Result<(), ChangeParamsError> {
        if self.is_username_free(model.username()).await {
            return Err(ChangeParamsError::UserDoesntExist);
        }

        for param in params {
            change_parametr(param, &model, self.0).await?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub enum ChangeParamQuery {
    UserTable(UserTableParams),
    School(i32),
    JobTitle(String),
    Class(Class),
}

async fn change_parametr(
    param: ChangeParamQuery,
    model: &UserModel,
    pool: &PgPool,
) -> Result<(), ChangeParamsError> {
    let sql = build_sql_string(calculate_table(param.clone(), model)?, param.clone());
    let mut query = sqlx::query(&sql).bind(model.username());
    query = match param.clone() {
        ChangeParamQuery::UserTable(param) => match param {
            UserTableParams::Password(password) => query.bind(password),
            UserTableParams::Email(email) => query.bind(email),
            UserTableParams::PhoneString(phone_number) => query.bind(phone_number),
            UserTableParams::FirstName(first_name) => query.bind(first_name),
            UserTableParams::LastName(last_name) => query.bind(last_name),
        },
        ChangeParamQuery::School(school) => query.bind(school),
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

fn calculate_table(
    param: ChangeParamQuery,
    model: &UserModel,
) -> Result<String, ChangeParamsError> {
    let table = match param {
        ChangeParamQuery::UserTable(_) => "users",
        ChangeParamQuery::School(_) => {
            if !model.user_specs().is_school_member() {
                return Err(ChangeParamsError::ChangingSchoolForNotSchoolMember);
            }
            match model.user_specs() {
                UserType::Teacher { subject, school } => "teachers",
                UserType::Pupil { class, school } => "students",
                UserType::Administrator { job_title, school } => "administrators",
                UserType::Other => panic!(),
            }
        }
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

fn build_sql_string(table: String, param: ChangeParamQuery) -> String {
    format!(
        "update {} where username=$1 set {};",
        table,
        match param {
            ChangeParamQuery::UserTable(param) => match param {
                UserTableParams::Password(_) => "password = $2",
                UserTableParams::Email(_) => "email = $2",
                UserTableParams::PhoneString(_) => "phone_number = $2",
                UserTableParams::FirstName(_) => "first_name = $2",
                UserTableParams::LastName(_) => "last_name = $2",
            },
            ChangeParamQuery::School(_) => "school = $2",
            ChangeParamQuery::JobTitle(_) => "job_title = $2",
            ChangeParamQuery::Class(_) => "class_num = $2, class_char = $3",
        }
    )
}

#[derive(Clone)]
pub enum UserTableParams {
    Password(String),
    Email(String),
    PhoneString(Option<String>),
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

pub enum ChangeParamsError {
    UserDoesntExist,
    ClassParametrChangingForNotStudent,
    ChangingSchoolForNotSchoolMember,
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
