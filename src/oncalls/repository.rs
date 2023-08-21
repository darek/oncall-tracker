use futures::stream::StreamExt;
use mongodb::{bson::doc, options::{FindOptions, UpdateModifications}, Client, Collection};

use crate::system::config::Mongo;
use crate::oncalls::structs::{OnCall, OnCallMonth, OnCallDay};

static ON_CALL_COLLECTION: &'static str = "OnCall";


pub async fn get_oncalls_years_for_user(client: Client, mongo: Mongo) -> Vec<i32> {
    let coll: Collection<OnCall> = client
        .database(mongo.database_name.as_str())
        .collection::<OnCall>(ON_CALL_COLLECTION);

    let distinct_on_calls = coll
        .distinct("year", None, None)
        .await
        .expect("could not load listings data.");

    return distinct_on_calls.iter().map(|year| year.as_i32().unwrap()).collect();
}

pub async fn list_oncalls_in_year_for_user(client: Client, mongo: Mongo, year: u16) -> Vec<u8> {
    let coll: Collection<OnCallMonth> = client
        .database(mongo.database_name.as_str())
        .collection::<OnCallMonth>(ON_CALL_COLLECTION);

    let mut options = FindOptions::default();
    options.sort = Some(doc! {
            "month": 1
        });
    options.projection = Some(doc! { "month": 1});
    let filter = doc! { "year": i32::from(year) };

    let mut cursor = coll
        .find(filter, options)
        .await
        .expect("could not load listings data.");

    let mut rows: Vec<u8> = Vec::new();

    while let Some(doc) = cursor.next().await {
        rows.push(doc.expect("could not load listings info.").month);
    }

    return rows;
}

pub async fn get_on_call_details(client: Client, mongo: Mongo, year: u16, month: u8) -> Option<OnCall> {
    let coll: Collection<OnCall> = client
        .database(mongo.database_name.as_str())
        .collection::<OnCall>(ON_CALL_COLLECTION);

    let filter = doc! { "year": i32::from(year), "month": i32::from(month) };

    return coll
        .find_one(filter, None)
        .await.expect("OnCall not found");
}

pub async fn save_oncall_day(client: Client, mongo: Mongo, year: u16, month: u8, new_day: OnCallDay) -> OnCallDay {
    let coll: Collection<OnCall> = client
        .database(mongo.database_name.as_str())
        .collection::<OnCall>(ON_CALL_COLLECTION);

    let filter = doc! { "year": i32::from(year), "month": i32::from(month) };

    let remove_day = doc! {
            "$pull": { "days": { "day": i32::from(new_day.day) } }
        };

    let add_day = doc! {
            "$push": { "days": bson::to_bson(&new_day).unwrap() }
        };

    let remove_commands = UpdateModifications::Document(remove_day);
    let add_commands = UpdateModifications::Document(add_day);

    coll
        .update_one(filter.clone(), remove_commands, None)
        .await
        .expect("could not load listings data.");

    coll
        .update_one(filter, add_commands, None)
        .await
        .expect("could not load listings data.");

    return new_day;
}
