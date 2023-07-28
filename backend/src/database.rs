use common::LoginData;
use serde::Deserialize;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

static DB: Surreal<Client> = Surreal::init();

#[derive(Deserialize, Debug)]
struct Record {
    id: Thing,
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

pub async fn query_login_data(ld: LoginData) -> Vec<LoginData> {
    let mut result = DB.query(format!(
                "SELECT * FROM type::table($table) WHERE username = '{}' AND password = '{}'",
                ld.username, ld.password
            )).bind(("table", "logins")).await.unwrap();
    result.take(0).unwrap()
}

pub async fn create_new_login(ld: LoginData) {
    let _record: Record = DB.create("logins").content(ld).await.unwrap();
}
