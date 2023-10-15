use chrono::Utc;
use common::LoginData;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Datetime, Duration, Thing};
use surrealdb::Surreal;

use crate::crypto::{encrypt, new_session_id, verify};

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

#[derive(Deserialize, Serialize)]
pub struct SessionDatabaseEntry {
    pub sid: String,
    pub username: String,
    pub expiration: Datetime,
}

pub struct DB {
    db: Surreal<Client>,
}

impl DB {
    pub async fn init() -> DB {
        let tmp = DB {
            db: Surreal::new::<Ws>("localhost:8001").await.unwrap()
        };
        tmp.db.signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .unwrap();
        tmp.db.use_ns("base_ns").use_db("base_db").await.unwrap();
        tmp
    }
    pub async fn query_username(& self, ld: &LoginData) -> Vec<LoginDatabaseEntry> {
        let mut result = self.db
            .query("SELECT * FROM type::table($table) WHERE username = $username")
            .bind(("table", "logins"))
            .bind(("username", ld.username.to_owned()))
            .await
            .unwrap();
        result.take(0).unwrap()
    }
    pub async fn check_login_data(& self, ld: &LoginData) -> Result<(), ()> {
        let users = self.query_username(&ld).await;
        match users.len() {
            1 => match verify(&ld.password, &(users[0].salt), &(users[0].pw_hash)) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            },
            _ => Err(()),
        }
    }
    pub async fn create_new_login(& self, ld: &LoginData) {
        let (salt, pw_hash) = encrypt(&ld.password);
        let entry = LoginDatabaseEntry {
            username: ld.username.clone(),
            pw_hash,
            salt,
        };
        let _record: Vec<LoginDatabaseEntry> = self.db.create("logins").content(entry).await.unwrap();
    }
    pub async fn query_sid(& self, sid: &String) -> Vec<SessionDatabaseEntry> {
        let mut result = self.db
            .query("SELECT * FROM type::table($table) WHERE sid = $sid")
            .bind(("table", "sessions"))
            .bind(("sid", sid))
            .await
            .unwrap();
        result.take(0).unwrap()
    }
    pub async fn create_new_session(& self, username: String) -> String {
        self.clean_up_old_sessions().await;
        let mut sid = new_session_id();
        while self.query_sid(&sid).await.len() != 0 {
            sid = new_session_id();
        }
        let entry = SessionDatabaseEntry {
            sid: sid.clone(),
            username,
            expiration: Duration(std::time::Duration::from_secs(10)) + Datetime(Utc::now()),
        };
        let _record: Vec<SessionDatabaseEntry> = self.db.create("sessions").content(entry).await.unwrap();
        sid
    }
    pub async fn clean_up_old_sessions(& self) {
        self.db.query("DELETE type::table($table) WHERE expiration < $ts")
            .bind(("table", "sessions"))
            .bind(("ts", Datetime(Utc::now())))
            .await
            .unwrap();
    }
}
