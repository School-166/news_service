use super::Controller;
use crate::{
    models::user::UserModel,
    prelude::Validateble,
    repositories::users::{
        queries::{ChangeQueryParam, GetByQueryParam},
        UserRepo,
    },
    types::Class,
    validators::repository_query::users::ValidationError,
};
use async_trait::async_trait;

pub struct UserController {
    username: String,
}

#[async_trait]
impl Controller for UserController {
    type Model = UserModel;
    async fn model(&self) -> UserModel {
        UserRepo::get_instance()
            .await
            .get_one_by(vec![GetByQueryParam::Username(self.username.clone())])
            .await
            .unwrap()
    }
}

impl UserController {
    pub fn from_model(model: UserModel) -> Self {
        UserController {
            username: model.username(),
        }
    }
    pub async fn change_name(&self, name: String) -> Result<(), Vec<ValidationError>> {
        UserRepo::get_instance()
            .await
            .change_params(
                vec![ChangeQueryParam::FirstName(name.clone()).validate(&self.model().await)?],
                self.model().await,
            )
            .await
            .unwrap();
        Ok(())
    }

    pub async fn change_last_name(&self, last_name: String) -> Result<(), Vec<ValidationError>> {
        UserRepo::get_instance()
            .await
            .change_params(
                vec![ChangeQueryParam::LastName(last_name.clone()).validate(&self.model().await)?],
                self.model().await,
            )
            .await
            .unwrap();
        Ok(())
    }

    pub async fn change_class(&self, class: Class) -> Result<(), Vec<ValidationError>> {
        UserRepo::get_instance()
            .await
            .change_params(
                vec![ChangeQueryParam::Class(class).validate(&self.model().await)?],
                self.model().await,
            )
            .await
            .expect("unreacheble");
        Ok(())
    }

    pub async fn change_about_me(&self, about: String) {
        UserRepo::get_instance()
            .await
            .change_params(
                vec![ChangeQueryParam::About(about.clone())
                    .validate(&self.model().await)
                    .unwrap()],
                self.model().await,
            )
            .await
            .unwrap();
    }
}
