use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Class {
    class_char: u8,
    class_num: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EditedState {
    Edited { edited_at: NaiveDateTime },
    NotEdited,
}

impl EditedState {
    pub fn is_edited(&self) -> bool {
        matches!(self, Self::Edited { .. })
    }
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
                (false, true) => ClassValidationError::WrongClassNumber { class_num },
                (true, false) => ClassValidationError::WrongClassChar { class_char },
                (false, false) => ClassValidationError::WrongClassCharEndNumber {
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
    WrongClassNumber { class_num: u8 },
    WrongClassChar { class_char: u8 },
    WrongClassCharEndNumber { class_char: u8, class_num: u8 },
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
            "PhysicalEducation" => Self::PhysicalEducation,
            "ComputerScience" => Self::ComputerScience,
            "Economics" => Self::Economics,
            "Law" => Self::Law,
            "Education" => Self::Education,
            _ => return Err(()),
        })
    }
}

impl ToString for Subject {
    fn to_string(&self) -> String {
        match self {
            Subject::Mathematics => "Mathematics",
            Subject::Physics => "Physics",
            Subject::Chemistry => "Chemistry",
            Subject::Biology => "Biology",
            Subject::Uzbek => "Uzbek",
            Subject::Russian => "Russian",
            Subject::English => "English",
            Subject::History => "History",
            Subject::Geography => "Geography",
            Subject::Literature => "Literature",
            Subject::PhysicalEducation => "Physical Education",
            Subject::ComputerScience => "Computer Science",
            Subject::Economics => "Economics",
            Subject::Law => "Law",
            Subject::Education => "Education",
        }
        .to_string()
    }
}
