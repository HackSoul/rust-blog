use diesel::prelude::*;
use super::db_connect;
use crate::entity::topic::Topic;
use crate::entity::schema::topics;

pub fn create_topic(topic: Topic) {

    let connection = &db_connect::db_connection();

    diesel::insert_into(topics::table)
        .values(&topic)
        .execute(connection)
        .expect("Error saving new topics");
}