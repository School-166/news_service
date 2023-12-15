use super::Model;
use crate::{
    controllers::users::UserController,
    types::{Class, Subject},
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserModel {
    username: String,
    about: String,
    first_name: String,
    last_name: String,
    password: String,
    email: String,
    phone_number: Option<String>,
    birth_date: NaiveDate,
    user_specs: UserType,
}

impl UserModel {
    pub fn username(&self) -> String {
        self.username.clone()
    }
    pub fn about(&self) -> String {
        self.about.clone()
    }
    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }
    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }
    pub fn password(&self) -> String {
        self.password.clone()
    }
    pub fn birth_date(&self) -> NaiveDate {
        self.birth_date.clone()
    }

    pub fn user_specs(&self) -> UserType {
        self.user_specs.clone()
    }

    pub fn phone_number(&self) -> Option<String> {
        self.phone_number.clone()
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserType {
    Teacher { subject: Subject },
    Student { class: Class },
    Administrator { job_title: String },
    Other,
}

impl UserType {
    pub fn is_administrator(&self) -> bool {
        if let Self::Administrator { job_title: _ } = self {
            return true;
        }
        false
    }

    pub fn is_student(&self) -> bool {
        if let Self::Student { class: _ } = self {
            return true;
        }
        false
    }

    pub fn is_school_member(&self) -> bool {
        if let Self::Other = self {
            return false;
        }
        true
    }

    pub fn is_teacher(&self) -> bool {
        if let Self::Teacher { subject: _ } = self {
            return true;
        }
        false
    }
}

impl Model for UserModel {
    type Controller = UserController;

    fn controller(self) -> Self::Controller {
        UserController::from_model(self)
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
