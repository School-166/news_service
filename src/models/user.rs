use std::cell::RefCell;

use sqlx::PgPool;

use super::{Controller, Model};

pub mod controlller;
pub mod implementation;
pub mod repository;

#[derive(Clone)]
pub struct UserModel {
    user_dto: UserDTO,
}

#[derive(Clone)]
pub struct UserDTO {
    username: String,
    first_name: String,
    last_name: String,
    password: String,
    user_specs: UserType,
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

    pub fn user_specs(&self) -> UserType {
        self.user_specs.clone()
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
    Administrator {
        job_title: String,
    },
    Other,
}

impl UserType {
    pub fn is_administrator(&self) -> bool {
        if let Self::Administrator { job_title: _ } = self {
            return true;
        }
        false
    }

    pub fn is_pupil(&self) -> bool {
        if let Self::Pupil {
            class_char: _,
            class_number: _,
            school: _,
        } = self
        {
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
        if let Self::Teacher { subject: _, school } = self {
            return true;
        }
        false
    }
}

impl Model for UserModel {
    type Controller = UserController;

    fn controller(&self) -> Self::Controller {
        UserController(RefCell::new(self.clone()))
    }
}

impl UserModel {
    pub fn username(&self) -> String {
        self.user_dto.username()
    }

    pub fn first_name(&self) -> String {
        self.user_dto.first_name()
    }
    pub fn last_name(&self) -> String {
        self.user_dto.last_name()
    }
    pub fn password(&self) -> String {
        self.user_dto.password()
    }

    pub fn user_specs(&self) -> UserType {
        self.user_dto.user_specs()
    }
}

impl Controller for UserController {
    type Model = UserModel;
    fn model(&self) -> UserModel {
        self.0.into_inner().clone()
    }
}

pub struct UserRepo(PgPool);
pub struct UserController(RefCell<UserModel>);
