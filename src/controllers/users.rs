use super::Controller;
use crate::{
    models::user::UserModel,
    prelude::Resource,
    repositories::users::{queries::ChangeQueryParam, UserRepo},
    validators::repository_query::users::{ValidatedChangeQueryParam, ValidationError},
};
use serde::Serialize;

pub struct UserController {
    username: String,
}

impl Controller for UserController {
    type Model = UserModel;
    async fn model(&self) -> UserModel {
        UserRepo::get_instance()
            .await
            .get_by_username(self.username.clone())
            .await
            .unwrap()
    }
}

#[derive(Serialize)]
pub enum SingError {
    WrongUsername,
    WrongPassword,
}

impl UserController {
    pub async fn sing(username: String, password: String) -> Result<Self, SingError> {
        match UserRepo::get_instance()
            .await
            .get_by_username(username)
            .await
        {
            Some(user) => {
                if password != user.password() {
                    return Err(SingError::WrongPassword);
                }
                Ok(UserController {
                    username: user.username(),
                })
            }
            None => Err(SingError::WrongUsername),
        }
    }

    pub async fn change_parameters(
        &self,
        params: Vec<ChangeQueryParam>,
    ) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        let mut validated_params = Vec::new();
        let model = self.model().await;

        for param in params {
            match ValidatedChangeQueryParam::validate(param, &model) {
                Ok(param) => validated_params.push(param),
                Err(mut err) => errors.append(&mut err),
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        UserRepo::get_instance()
            .await
            .change_params(validated_params, self.model().await)
            .await;
        Ok(())
    }

    pub async fn comment(&self, resource: Box<dyn Resource>, content: String) {
        resource.comment(content, &self);
    }

    pub async fn like(&self, markable: Box<dyn Resource>) {
        markable.like(&self)
    }

    pub async fn dislike(&self, markable: Box<dyn Resource>) {
        markable.dislike(&self)
    }
}
