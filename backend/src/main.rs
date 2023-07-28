use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use rocket::{get, launch, post, routes};
use std::path::PathBuf;

use common::from_str;

mod database;

#[post("/api/register", data = "<incoming>")]
async fn register(incoming: String) -> String {
    match from_str(&incoming) {
        Some(login) => {
            let list = database::query_login_data(login.clone()).await;
            match list.len() {
                0 => {
                    database::create_new_login(login).await;
                    "Yay!".to_owned()
                },
                _ => "Error".to_owned(),
            }
        }
        None => "Error".to_owned(),
    }
}

#[post("/api/login", data = "<incoming>")]
async fn login(incoming: String) -> String {
    match from_str(&incoming) {
        Some(login) => {
            let list = database::query_login_data(login).await;
            match list.len() {
                1 => "Yay!".to_owned(),
                _ => "Error".to_owned(),
            }
        }
        None => "Error".to_owned(),
    }
}

// Return the index file as a Rocket NamedFile
async fn get_index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("../frontend/dist/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

//Create a route for any url that is a path from the /
#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("../frontend/dist").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}

// Return the index when the url is /
#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    get_index().await
}

#[launch]
async fn rocket() -> _ {
    database::init().await;
    rocket::build().mount("/", routes![index, static_files, login, register])
}
