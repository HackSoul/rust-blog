use super::schema::topics;
use std::time::SystemTime;

#[derive(Insertable)]
#[table_name="topics"]
pub struct Topic {
    pub name: String,
    pub create_date: SystemTime
}