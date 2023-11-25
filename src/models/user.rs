use std::{cell::RefCell, str::FromStr};

use actix_web::body::to_bytes;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use self::controlller::Class;

use super::{Controller, Model};

pub mod controlller;
pub mod implementation;
pub mod repository;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserModel {
    username: String,
    first_name: String,
    last_name: String,
    password: String,
    email: String,
    phone_number: Option<String>,
    user_specs: UserType,
}

impl UserModel {
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

    pub fn phone_number(&self) -> Option<String> {
        self.phone_number.clone()
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Subject {
    Mathematics,
    Physics,
    Chemistry,
    Biology,
    Uzbek,
    Russian,
    English,
    History,
    Geography,
    Literature,
    PhysicalEducation,
    ComputerScience,
    Economics,
    Law,
    Education,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum UserType {
    Teacher { subject: Subject, school: i32 },
    Pupil { class: Class, school: i32 },
    Administrator { job_title: String, school: i32 },
    Other,
}

impl UserType {
    pub fn is_administrator(&self) -> bool {
        if let Self::Administrator {
            job_title: _,
            school: _,
        } = self
        {
            return true;
        }
        false
    }

    pub fn is_pupil(&self) -> bool {
        if let Self::Pupil {
            class: _,
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
        if let Self::Teacher {
            subject: _,
            school: _,
        } = self
        {
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

impl Controller for UserController {
    type Model = UserModel;
    fn model(&self) -> UserModel {
        self.0.clone().into_inner().clone()
    }
}

pub struct UserRepo(&'static PgPool);
pub struct UserController(RefCell<UserModel>);
impl FromStr for Subject {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Mathematics" => Self::Mathematics,
            "Physics" => Self::Physics,
            "Chemistry" => Self::Chemistry,
            "Biology" => Self::Biology,
            "Uzbek" => Self::Uzbek,
            "Russian" => Self::Russian,
            "English" => Self::English,
            "History" => Self::History,
            "Geography" => Self::Geography,
            "Literature" => Self::Literature,
            "Physical Education" => Self::PhysicalEducation,
            "Computer Science" => Self::ComputerScience,
            "Economics" => Self::Economics,
            "Law" => Self::Law,
            "Education" => Self::Education,
            _ => return Err(()),
        })
    }
}
