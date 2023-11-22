use super::UserController;

impl UserController {
    pub fn uuid(&self) -> String {
        self.0.uuid.clone()
    }

    pub fn username(&self)->String{
        self.0.user_dto.username()
    }

    pub fn 
}
