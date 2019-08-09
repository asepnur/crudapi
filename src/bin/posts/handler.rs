use actix_web::{
    web,  HttpResponse, error::BlockingError
};
use futures::Future;

use super::models;
use super::service;
use super::errors::ServiceError;

pub fn get_posts(
    pool: web::Data<models::Pool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || service::show_post(pool)).then(
        move |res: Result<Vec<models::Post>, BlockingError<ServiceError>>| match res {
            Ok(post) => {
                Ok(HttpResponse::Ok().json(post))
            }
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                BlockingError::Canceled => Err(ServiceError::InternalServerError),
            },
        },
    )
}

pub fn create_post(
    data: web::Json<models::NewPostPayload>,
    pool: web::Data<models::Pool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || service::create_post(pool, data)).then(
        move |res: Result<models::Post, BlockingError<ServiceError>>| match res {
            Ok(post) => {
                Ok(HttpResponse::Ok().json(post))
            }
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                BlockingError::Canceled => Err(ServiceError::InternalServerError),
            },
        },
    )
}