use crate::models::posts::ActiveModel as PostActiveModel;
use crate::models::posts::Entity as Post;
use crate::models::posts::Model as PostModel;
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::DeleteResult;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PostDto {
    pub id: i32,
    pub title: String,
    pub fact: String,
}
impl From<PostModel> for PostDto {
    fn from(value: PostModel) -> Self {
        Self {
            id: value.id,
            title: value.title,
            fact: value.fact,
        }
    }
}

impl From<PostActiveModel> for PostDto {
    fn from(value: PostActiveModel) -> Self {
        Self {
            id: value.id.unwrap(),
            title: value.title.unwrap(),
            fact: value.fact.unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreatePostDto {
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct CatFactDto {
    fact: String,
    length: u32,
}

pub async fn get_all(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<PostDto>>, StatusCode> {
    let rows: Vec<PostModel> = Post::find().all(&db).await.unwrap();
    let users: Vec<PostDto> = rows.into_iter().map(PostDto::from).collect();
    Ok(Json(users))
}

pub async fn create_post(
    State(db): State<DatabaseConnection>,
    Json(new_post): Json<CreatePostDto>,
) -> Result<Json<PostDto>, StatusCode> {
    let resp = reqwest::get("https://catfact.ninja/fact")
        .await
        .unwrap()
        .json::<CatFactDto>()
        .await
        .unwrap();

    let created_post = PostActiveModel {
        title: Set(new_post.title.to_string()),
        fact: Set(resp.fact.to_string()),
        ..Default::default()
    }
    .save(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(created_post.into()))
}

pub async fn get_post(
    State(db): State<DatabaseConnection>,
    Path(post_id): Path<i32>,
) -> Result<Json<PostDto>, StatusCode> {
    let post: Option<PostModel> = Post::find_by_id(post_id).one(&db).await.unwrap();
    if let Some(post) = post {
        return Ok(Json(post.into()));
    }
    Err(StatusCode::NOT_FOUND)
}

pub async fn delete_post(
    State(db): State<DatabaseConnection>,
    Path(post_id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result: DeleteResult = Post::delete_by_id(post_id).exec(&db).await.unwrap();

    if result.rows_affected == 1 {
        return Ok(StatusCode::CREATED);
    }
    Err(StatusCode::NOT_FOUND)
}
