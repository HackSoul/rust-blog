use actix_web::{web, Result};
use serde_derive::{Serialize, Deserialize};
use crate::dao::topic_repository;
use crate::entity::topic;
use actix_web::http::header::Date;
use std::time::SystemTime;
use diesel::select;

#[derive(Serialize)]
pub struct MyObj {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    name: String,
}

pub fn create_topic(info: web::Query<Info>) -> Result<web::Json<MyObj>> {
    topic_repository::create_topic(topic::Topic{
        name: String::from(&info.name),
        create_date: SystemTime::now(),
    });
    Ok(web::Json(MyObj{name: String::from(&info.name)}))
}