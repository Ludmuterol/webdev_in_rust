use common::LoginData;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::crypto::{encrypt, verify};

static DB: Surreal<Client> = Surreal::init();

#[derive(Deserialize, Debug)]
struct Record {
    id: Thing,
}

#[derive(Deserialize, Serialize)]
pub struct DatabaseEntry {
    username: String,
    pw_hash: String,
    salt: String,
}

pub async fn init() {
    DB.connect::<Ws>("127.0.0.1:8001").await.unwrap();
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    DB.use_ns("base_ns").use_db("base_db").await.unwrap();
}

pub async fn query_username(ld: LoginData) -> Vec<DatabaseEntry> {
    let mut result = DB.query(format!(
                "SELECT * FROM type::table($table) WHERE username = '{}'",
                ld.username,
            )).bind(("table", "logins")).await.unwrap();
    result.take(0).unwrap()
}

pub async fn check_login_data(ld: LoginData) -> Result<(),()> {
    let users = query_username(ld.clone()).await;
    match users.len() {
        1 => {
            match verify(ld.password, &(users[0].salt), &(users[0].pw_hash)) {
                Ok(_) => Ok(()),
                Err(_) => Err(())
            }
        },
        _ => Err(())
    }
}

pub async fn create_new_login(ld: LoginData) {
    let (salt, pw_hash) = encrypt(ld.password);
    let entry = DatabaseEntry {username: ld.username, pw_hash, salt};
    let _record: Record = DB.create("logins").content(entry).await.unwrap();
}
