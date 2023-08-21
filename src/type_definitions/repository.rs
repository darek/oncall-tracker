use futures::stream::StreamExt;
use mongodb::{Client, Collection};
use mongodb::{bson::doc};
use mongodb::options::{UpdateModifications, UpdateOptions};

use crate::system::config::Mongo;
use crate::type_definitions::structs::{TypeDefinitionDetails, TypeDefinitionGroup};

static TYPE_DEFINITION_GROUP_COLLECTION: &'static str = "TypeDefinitionGroup";

pub async fn get_type_definition_groups(client: Client, mongo: Mongo) -> Vec<TypeDefinitionGroup> {

    let coll: Collection<TypeDefinitionGroup> = client
        .database(mongo.database_name.as_str())
        .collection::<TypeDefinitionGroup>(TYPE_DEFINITION_GROUP_COLLECTION);

    let mut cursor = coll
        .find(None, None)
        .await
        .expect("could not load listings data.");

    // Iterate over the results of the cursor.
    let mut definitions = Vec::new();
    while let Some(result) = cursor.next().await {
        definitions.push(result.expect("could not load listings info."))
    }
    return definitions;
}

pub async fn save_type_definition_group(client: Client, mongo: Mongo, type_def_group: TypeDefinitionGroup) -> TypeDefinitionGroup {
    let coll: Collection<TypeDefinitionGroup> = client
        .database(mongo.database_name.as_str())
        .collection::<TypeDefinitionGroup>(TYPE_DEFINITION_GROUP_COLLECTION);

    coll.insert_one(type_def_group.clone(), None)
        .await
        .expect("could not load listings data.");

    return type_def_group
}

pub async fn insert_type_definition_detail(client: Client, mongo: Mongo, type_definition_id: String, definition_detail: TypeDefinitionDetails) -> TypeDefinitionDetails {
    let coll: Collection<TypeDefinitionGroup> = client
        .database(mongo.database_name.as_str())
        .collection::<TypeDefinitionGroup>(TYPE_DEFINITION_GROUP_COLLECTION);

    let filter = doc! { "_id": type_definition_id };
    let add_doc = doc! {
        "$push": { "definitions": bson::to_bson(&definition_detail.clone()).unwrap() }
    };

    let add_command = UpdateModifications::Document(add_doc);

    coll.update_one(filter,add_command, None)
        .await
        .expect("Unable to save type details");

    definition_detail.clone()
}

pub async fn save_type_definition_detail(client: Client, mongo: Mongo, type_group_id: String, type_detail_id: String, definition_detail: TypeDefinitionDetails) -> String {
    let coll: Collection<TypeDefinitionGroup> = client
        .database(mongo.database_name.as_str())
        .collection::<TypeDefinitionGroup>(TYPE_DEFINITION_GROUP_COLLECTION);

    let filter = doc! { "_id": type_group_id };
    let update_doc = doc! {
        "$set": {
            "definitions.$[definition].name": definition_detail.name,
            "definitions.$[definition].base": definition_detail.base,
            "definitions.$[definition].multiplier": definition_detail.multiplier
        }
    };
    let mut update_filter = Vec::new();
    update_filter.push(doc!{
        "definition.id": type_detail_id
    });
    let update_command = UpdateModifications::Document(update_doc);
    let update_options = Some(UpdateOptions::builder().array_filters(update_filter).build());
    coll
        .update_one(filter, update_command, update_options)
        .await
        .expect("Unable to update definition detail");

    return "OK".to_string();
}

pub async fn remove_type_definition_detail(client: Client, mongo: Mongo, type_def_id: String, type_def_detail_id: String) -> String {
    let coll: Collection<TypeDefinitionGroup> = client
        .database(mongo.database_name.as_str())
        .collection::<TypeDefinitionGroup>(TYPE_DEFINITION_GROUP_COLLECTION);

    let filter = doc! { "_id": type_def_id };

    let remove_detail = doc! {
        "$pull": { "definitions": { "id": type_def_detail_id } }
    };

    let remove_command = UpdateModifications::Document(remove_detail);

    coll
        .update_one(filter, remove_command, None)
        .await
        .expect("Unable to remove definition detail");

    "OK".to_string()
}