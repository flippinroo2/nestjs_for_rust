use nidrs::externs::axum;
use nidrs::openapi::api;
use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    Json,
};
use nidrs::{post, AppResult, Inject};
use nidrs_macro::{controller, get};

use crate::models::entities::user::User;

use super::{dto::CreateUserDto, service::UserService};

#[controller("/user")]
pub struct UserController {
    user_service: Inject<UserService>,
}

impl UserController {
    #[get("/")]
    pub async fn get_user_all(&self, Query(q): Query<HashMap<String, String>>) -> AppResult<Json<Vec<User>>> {
        println!("Query {:?}", q);

        Ok(Json(self.user_service.all().await?))
    }

    #[get("/:id")]
    pub async fn get_user_by_id(&self, Path(user_id): Path<u32>, Query(q): Query<HashMap<String, String>>) -> AppResult<Json<User>> {
        println!("Query {:?}", q);

        Ok(Json(self.user_service.find_by_id(user_id).await?))
    }

    #[api]
    #[post("/")]
    pub async fn create_user(&self, Json(j): Json<CreateUserDto>) -> AppResult<String> {
        println!("Query {:?}", j);

        Ok(self.user_service.create(j).await?.to_string())
    }
}
