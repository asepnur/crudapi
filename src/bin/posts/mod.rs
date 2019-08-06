// pub mod schema;
pub mod schema;
pub mod models;


extern crate diesel;
extern crate dotenv;
use actix_web::{web};

use self::diesel::prelude::*;

pub fn show_post(pool: web::Data<models::Pool>) -> Vec<models::Post>{
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
    posts_vec
} 
use models::{Post, NewPost};
pub fn create_post(pool: web::Data<models::Pool>, new_post:NewPost) -> Post {
    use schema::posts;
    let connection = pool.get().unwrap();
    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&*connection)
        .expect("Error saving new post")
}