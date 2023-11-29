use super::{
    repository::queries::{validation::ValidationError, ChangeQueryParam, Validateble},
    UserController, UserRepo,
};
use crate::models::Controller;
use serde::{Deserialize, Serialize};

impl UserController {
    pub fn change_name(&self, _name: String) -> Result<(), ()> {
        todo!()
    }

    pub fn change_last_name(&self, _last_name: String) -> Result<(), ()> {
        todo!()
    }

    pub async fn change_class(&self, class: Class) -> Result<(), Vec<ValidationError>> {
        UserRepo::get_instance()
            .await
            .change_params(
                vec![ChangeQueryParam::Class(class).validate(&self.model())?],
                self.model(),
            )
            .await
            .expect("unreacheble");
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Class {
    class_char: u8,
    class_num: u8,
}

impl Class {
    pub fn from(class_char: u8, class_num: u8) -> Result<Class, ClassValidationError> {
        Err(
            match (
                (1..=11).contains(&class_num),
                class_char.is_ascii_alphabetic(),
            ) {
                (true, true) => {
                    return Ok(Class {
                        class_char,
                        class_num,
                    })
                }
                (false, true) => ClassValidationError::UnableClassNumber { class_num },
                (true, false) => ClassValidationError::UnableClassChar { class_char },
                (false, false) => ClassValidationError::UnableClassCharEndNumber {
                    class_char,
                    class_num,
                },
            },
        )
    }

    pub fn class_char(&self) -> String {
        String::from_utf8(vec![self.class_char]).unwrap()
    }

    pub fn class_num(&self) -> u8 {
        self.class_num
    }
}

#[derive(Debug)]
pub enum ClassValidationError {
    UnableClassNumber { class_num: u8 },
    UnableClassChar { class_char: u8 },
    UnableClassCharEndNumber { class_char: u8, class_num: u8 },
}
