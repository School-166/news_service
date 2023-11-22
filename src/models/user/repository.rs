use sqlx::{postgres::PgRow, FromRow};

use super::{User, UserController, UserDTO, UserRepo};

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
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        todo!()
    }
}
