use mongodb::{bson::doc, Client, Collection};
use crate::system::config::Mongo;
use crate::user::struts::User;

static USER_COLLECTION: &'static str = "User";

pub async fn get_user_by_username(client: Client, mongo: Mongo, username: String) -> Option<User> {

    let coll: Collection<User> = client
        .database(mongo.database_name.as_str())
        .collection::<User>(USER_COLLECTION);

    let filter = doc! { "username": username };
    
    return coll
        .find_one(filter, None)
        .await
        .expect("could not load listings data.");
}

pub async fn insert_new_user(client: Client, mongo: Mongo, user: User) -> User {
    let coll: Collection<User> = client
        .database(mongo.database_name.as_str())
        .collection::<User>(USER_COLLECTION);

    coll.insert_one(user.clone(), None)
        .await
        .expect("could not load listings data.");

    return user
}