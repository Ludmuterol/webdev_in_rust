use chrono::Utc;
use common::LoginData;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Datetime, Duration, Thing};
use surrealdb::Surreal;

use crate::crypto::{encrypt, new_session_id, verify};

static DB: Surreal<Client> = Surreal::init();

#[derive(Deserialize, Debug)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Deserialize, Serialize)]
pub struct LoginDatabaseEntry {
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

pub async fn query_username(ld: &LoginData) -> Vec<LoginDatabaseEntry> {
    let mut result = DB
        .query("SELECT * FROM type::table($table) WHERE username = $username")
        .bind(("table", "logins"))
        .bind(("username", ld.username.to_owned()))
        .await
        .unwrap();
    result.take(0).unwrap()
}

pub async fn check_login_data(ld: &LoginData) -> Result<(), ()> {
    let users = query_username(&ld).await;
    match users.len() {
        1 => match verify(&ld.password, &(users[0].salt), &(users[0].pw_hash)) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        },
        _ => Err(()),
    }
}

pub async fn create_new_login(ld: &LoginData) {
    let (salt, pw_hash) = encrypt(&ld.password);
    let entry = LoginDatabaseEntry {
        username: ld.username.clone(),
        pw_hash,
        salt,
    };
    let _record: Record = DB.create("logins").content(entry).await.unwrap();
}

#[derive(Deserialize, Serialize)]
pub struct SessionDatabaseEntry {
    pub sid: String,
    pub username: String,
    pub expiration: Datetime,
}

pub async fn query_sid(sid: &String) -> Vec<SessionDatabaseEntry> {
    let mut result = DB
        .query("SELECT * FROM type::table($table) WHERE sid = $sid")
        .bind(("table", "sessions"))
        .bind(("sid", sid))
        .await
        .unwrap();
    result.take(0).unwrap()
}

pub async fn create_new_session(username: String) -> String {
    let mut sid = new_session_id();
    while query_sid(&sid).await.len() != 0 {
        sid = new_session_id();
    }
    let entry = SessionDatabaseEntry {
        sid: sid.clone(),
        username,
        expiration: Duration(std::time::Duration::from_secs(10)) + Datetime(Utc::now()),
    };
    let _record: Record = DB.create("sessions").content(entry).await.unwrap();
    sid
}

pub async fn clean_up_old_sessions() {
    DB.query("DELETE type::table($table) WHERE expiration < $ts")
        .bind(("table", "sessions"))
        .bind(("ts", Datetime(Utc::now())))
        .await
        .unwrap();
}
