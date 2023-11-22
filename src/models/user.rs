use sqlx::PgPool;

pub mod controlller;
pub mod implementation;
pub mod repository;

pub struct User {
    pub uuid: String,
    pub user_dto: UserDTO,
}

pub struct UserDTO {
    username: String,
    first_name: String,
    last_name: String,
    password: String,
    user_type: UserType,
}

impl UserDTO {
    pub fn username(&self) -> String {
        self.username.clone()
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

    pub fn user_type(&self) -> UserType {
        self.user_type.clone()
    }
}

#[derive(Clone)]
pub enum Subject {}

#[derive(Clone)]
pub enum UserType {
    Teacher {
        subject: Subject,
        school: u16,
    },
    Pupil {
        class_char: char,
        class_number: u8,
        school: u16,
    },
    Other,
}

pub struct UserRepo(PgPool);
pub struct UserController(User);
