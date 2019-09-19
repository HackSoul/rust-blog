use diesel::sql_types::Date;
use super::schema::topics;

#[derive(Insertable)]
#[table_name="topics"]
pub struct Topic {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}