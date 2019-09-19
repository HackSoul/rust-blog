use actix_web::{web, Result};
use serde_derive::{Serialize, Deserialize};

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
    Ok(web::Json(MyObj{name: String::from(&info.name)}))
}