
use diesel::{r2d2::ConnectionManager, PgConnection};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use super::schema::posts;
#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name="posts"]
pub struct NewPost<'a>{
    pub title: &'a str,
    pub body: &'a str,
}
// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;