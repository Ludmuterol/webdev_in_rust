use rocket::fs::NamedFile;
use rocket::http::{CookieJar, Cookie};
use rocket::outcome::IntoOutcome;
use rocket::request::FromRequest;
use rocket::response::status::NotFound;
use rocket::{get, launch, post, routes};
use std::path::PathBuf;

use common::from_str;

mod database;
mod crypto;

#[post("/api/register", data = "<incoming>")]
async fn register(incoming: String) -> String {
    match from_str(&incoming) {
        Some(login) => {
            let list = database::query_username(login.clone()).await;
            match list.len() {
                0 => {
                    database::create_new_login(login).await;
                    "Ok".to_owned()
                },
                _ => "Error".to_owned(),
            }
        }
        None => "Error".to_owned(),
    }
}

#[post("/api/login", data = "<incoming>")]
async fn login(jar: &CookieJar<'_>, incoming: String) -> String {
    match from_str(&incoming) {
        Some(login) => {
            let res = database::check_login_data(login).await;
            match res {
                Ok(_) => {
                    jar.add_private(Cookie::new("user_id", "1".to_owned()));
                    "Ok".to_owned()
                },
                Err(_) => "Error".to_owned(),
            }
        }
        None => "Error".to_owned(),
    }
}

#[get("/api/logout")]
async fn logout(jar: &CookieJar<'_>) -> String {
    jar.remove_private(Cookie::named("user_id"));
    "Ok".to_owned()
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

struct User(usize);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
       type Error = std::convert::Infallible;
       async fn from_request(request: &'r rocket::Request<'_>) ->  rocket::request::Outcome<User,Self::Error> {
           request.cookies().get_private("user_id").and_then(|cookie| cookie.value().parse().ok())
               .map(User)
               .or_forward(())
       }
}

#[get("/secret")]
fn secret(_user: User) -> String {
    "Secret!".to_owned()
}

#[launch]
async fn rocket() -> _ {
    database::init().await;
    rocket::build().mount("/", routes![index, static_files, login, register, logout, secret])
}
