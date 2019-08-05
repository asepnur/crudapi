extern crate diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection(connection_string: String) -> PgConnection {
    PgConnection::establish(&connection_string)
        .expect(&format!("Error connection to {}", connection_string))
}