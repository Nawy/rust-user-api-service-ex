use crate::models::users::ActiveModel as UserActiveModel;
use crate::models::users::Entity as User;
use crate::models::users::Model as UserModel;
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserDto {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub fact: String,
}
impl From<UserModel> for UserDto {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id,
            email: value.email,
            name: value.name,
            fact: value.fact,
        }
    }
}

impl From<UserActiveModel> for UserDto {
    fn from(value: UserActiveModel) -> Self {
        Self {
            id: value.id.unwrap(),
            email: value.email.unwrap(),
            name: value.name.unwrap(),
            fact: value.fact.unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CatFactDto {
    fact: String,
    length: u32,
}

pub async fn get_all(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<UserDto>>, StatusCode> {
    let rows: Vec<UserModel> = User::find().all(&db).await.unwrap();
    let users: Vec<UserDto> = rows.into_iter().map(UserDto::from).collect();
    Ok(Json(users))
}

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(new_user): Json<CreateUserDto>,
) -> Result<Json<UserDto>, StatusCode> {
    let resp = reqwest::get("https://catfact.ninja/fact")
        .await
        .unwrap()
        .json::<CatFactDto>()
        .await
        .unwrap();

    let create_user = UserActiveModel {
        email: Set(new_user.email.to_string()),
        name: Set(new_user.name.to_string()),
        fact: Set(resp.fact.to_string()),
        ..Default::default()
    }
    .save(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(create_user.into()))
}

pub async fn delete_user() -> Result<Json<UserDto>, StatusCode> {
    Err(StatusCode::NOT_FOUND)
}
