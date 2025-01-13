use std::collections::HashMap;

use nidrs::externs::axum::{
    extract::{Path, Query},
    http::HeaderMap,
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
    pub async fn get_user_all(&self, header: HeaderMap) -> AppResult<Json<Vec<User>>> {
        // println!("Query {:?}", q);

        let rid = header.get("X-RID");

        if let Some(rid) = rid {
            println!("rid: {:?}", rid);
        }

        // AppResult::Ok(self.user_service.all().await?).header("X-Test", "test");
        Ok(Json(self.user_service.all().await?))
    }

    #[get("/:id")]
    pub async fn get_user_by_id(&self, Path(user_id): Path<i32>, Query(q): Query<HashMap<String, String>>) -> AppResult<Json<User>> {
        println!("Query {:?}", q);

        Ok(Json(self.user_service.find_by_id(user_id).await?))
    }

    #[post("/")]
    pub async fn create_user(&self, Json(j): Json<CreateUserDto>) -> AppResult<Json<usize>> {
        println!("Query {:?}", j);

        Ok(Json(self.user_service.create(j).await?))
    }
}
