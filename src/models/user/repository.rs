use sqlx::{postgres::PgRow, Acquire, FromRow};

use crate::models::Model;

use super::{User, UserController, UserDTO, UserModel, UserRepo};

impl UserRepo {
    pub fn new() -> Self {
        todo!()
    }

    pub fn register_user(&self, user_dto: UserDTO) -> Result<UserController, sqlx::Error> {
        todo!()
    }

    pub fn get_by_uuid(&self, uuid: String) -> Option<UserController> {
        todo!()
    }

    pub fn get_by_username(&self, username: String) -> Option<UserController> {
        todo!()
    }

    pub fn is_username_free(&self, username: String) -> bool {
        todo!()
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
                ChangeParamQuery::Class {
                    class_number,
                    class_char,
                } => {
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
    Class { class_number: u8, class_char: u8 },
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        todo!()
    }
}
