extern crate diesel;
extern crate dotenv;
use actix_web::{web};
use super::models;
use super::schema;
use super::errors::ServiceError;

use self::diesel::prelude::*;
use models::{Post, NewPost};

pub fn show_post(pool: web::Data<models::Pool>) -> Result<Vec<models::Post>, ServiceError>{
    let conn = &pool.get().unwrap();
    use schema::posts::dsl::*;
    let result = posts
        .filter(published.eq(true))
        .load::<models::Post>(&*conn)
        .expect("Error loading post");

    let mut posts_vec:Vec<models::Post> = Vec::new();
    for ps in result {
        let post = models::Post{
            id: ps.id,
            title: ps.title,
            body: ps.body,
            published: ps.published,
        };
        posts_vec.push(post);
    }
    Ok(posts_vec.into())
}

pub fn create_post(pool: web::Data<models::Pool>, data: web::Json<models::NewPostPayload>) -> Result<Post, ServiceError> {
    use schema::posts;
    let connection = pool.get().unwrap();
    let new_post = NewPost{
        title: &data.0.title,
        body: &data.0.body,
    };
    let post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&*connection)
        .expect("Error saving new post");
    Ok(post)
}
