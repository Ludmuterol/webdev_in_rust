use rocket::{routes, launch, get, post};
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::PathBuf;

use common::from_str;

#[post("/api/register", data="<incoming>")]
async fn upload(incoming: String) -> String {
    match from_str(&incoming) {
        Some(login) => login.username.to_owned(),
        None => "Error".to_owned()
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
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, static_files, upload])
}
