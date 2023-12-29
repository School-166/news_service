use super::Controller;
use crate::{
    models::{user::UserModel, Commentable, Model},
    prelude::{Markable, Validateble},
    repositories::users::{queries::ChangeQueryParam, UserRepo},
    types::Class,
    validators::repository_query::users::ValidationError,
};
use serde::Serialize;
use uuid::Uuid;

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

impl Model for UserModel {
    type Controller = UserController;

    fn controller(self) -> Self::Controller {
        UserController {
            username: self.username(),
        }
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

    pub async fn sing_by_uuid(uuid: Uuid) -> Option<Self> {
        if let Some(user) = UserRepo::get_instance().await.get_by_uuid(uuid).await {
            return Some(UserController {
                username: user.username(),
            });
        }
        None
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

    pub async fn comment(&self, resource: impl Commentable + Sync, content: String) {
        resource.comment(content, self.model().await).await
    }

    pub async fn like(&self, markable: impl Markable) {
        markable.like(self.model().await).await
    }

    pub async fn dislike(&self, markable: impl Markable) {
        markable.dislike(self.model().await).await
    }
}
