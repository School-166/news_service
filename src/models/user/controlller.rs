use super::UserController;

impl UserController {
    pub fn change_name(&self, name: String) -> Result<(), ()> {
        todo!()
    }

    pub fn change_last_name(&self, last_name: String)->Result<(), ()>{
        todo!()
    }

    pub fn change_class(&self, class: Class)->Result<(), ChangeClassError>{
        todo!()
    }

}

pub struct Class{
    class_char: u8,
    class_num: u8
}

impl Class{
    pub fn from(class_char: u8, class_num:u8)->Result<Class, ClassValidationError>{
        Err(match  ((1..=11).contains(class_num), class_char.is_ascii_alphabetic()){
            (true, true) => return Ok(Class { class_char, class_num }),
            (false, true) => ClassValidationError::UnableClassNumber { class_num },
            (true, false) => ClassValidationError::UnableClassChar { class_char },
            (false, false) => ClassValidationError::UnableClassCharEndNumber { class_char, class_num },
        })
    }

    pub fn class_char(&self)->String{
        String::from_utf8(vec![self.class_char]).unwrap()
    }

pub fn class_num(&self)->u8{
    self.class_num
}
}

pub enum ClassValidationError{
    UnableClassNumber{class_num: u8},
    UnableClassChar{class_char: u8},
    UnableClassCharEndNumber{class_char: u8, class_num:u8}
}