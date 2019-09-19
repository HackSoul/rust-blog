use actix_web::{web, Result};
use serde_derive::{Serialize, Deserialize};
use crate::dao::topic_repository;
use crate::entity::topic;

#[derive(Serialize)]
pub struct MyObj {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    name: String,
}

pub fn index(info: web::Query<Info>) -> Result<web::Json<MyObj>> {
    String::from("aaa");
    topic_repository::create_topic(topic::Topic{
        id: 1,
        title: String::from("title"),
        body: String::from("body"),
        published: true
    });
    Ok(web::Json(MyObj{name: String::from(&info.name)}))
}