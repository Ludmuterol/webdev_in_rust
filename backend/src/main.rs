use chrono::Utc;
use database::{create_new_session, query_sid};
use pwned::api::*;
use rocket::fs::NamedFile;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::outcome::IntoOutcome;
use rocket::request::FromRequest;
use rocket::response::status::NotFound;
use rocket::{get, launch, post, routes};
use std::path::PathBuf;
use surrealdb::sql::Datetime;

use common::{LoginData, ProfileData};

mod crypto;
mod database;

#[post("/api/register", data = "<incoming>")]
async fn register(incoming: String) -> String {
    match LoginData::from_str(&incoming) {
        Some(login) => {
            let login = LoginData {
                username: login.username.to_lowercase(),
                password: login.password,
            };
            let list = database::query_username(&login).await;
            match list.len() {
                0 => {
                    let pwned = PwnedBuilder::default().build().unwrap();
                    match pwned.check_password(login.password.clone()).await {
                        Ok(pwd) => match pwd.found {
                            true => {
                                return "Error: This is a known Password!".to_owned();
                            }
                            false => {
                                database::create_new_login(&login).await;
                                return "Ok".to_owned();
                            }
                        },
                        Err(e) => {
                            println!("Error: {}", e);
                            "Error".to_owned()
                        }
                    }
                }
                _ => "Username already taken".to_owned(),
            }
        }
        None => "Error".to_owned(),
    }
}

#[post("/api/login", data = "<incoming>")]
async fn login(jar: &CookieJar<'_>, incoming: String) -> String {
    match LoginData::from_str(&incoming) {
        Some(login) => {
            let login = LoginData {
                username: login.username.to_lowercase(),
                password: login.password,
            };
            let res = database::check_login_data(&login).await;
            match res {
                Ok(_) => {
                    let mut c = Cookie::new("id", create_new_session(login.username).await);
                    c.set_secure(true);
                    c.set_http_only(true);
                    c.set_same_site(SameSite::Strict);
                    c.set_max_age(None);
                    c.set_expires(None);
                    jar.add_private(c);
                    "Ok".to_owned()
                }
                Err(_) => "Error".to_owned(),
            }
        }
        None => "Error".to_owned(),
    }
}

#[get("/api/logout")]
async fn logout(jar: &CookieJar<'_>) -> String {
    jar.remove_private(Cookie::named("id"));
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

struct User;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;
    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<User, Self::Error> {
        match request.cookies().get_private("id") {
            Some(cookie) => match cookie.value().parse::<String>().ok() {
                Some(sid) => match query_sid(&sid).await {
                    i if i.len() == 1 => {
                        if i[0].expiration > Datetime(Utc::now()) {
                            Some(User)
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                None => None,
            },
            None => None,
        }
        .or_forward(())
    }
}

#[get("/secret")]
fn secret(_user: User) -> String {
    "Secret!".to_owned()
}

#[get("/api/profile")]
async fn profile(_user: User, jar: &CookieJar<'_>) -> String {
    let var = jar.get_private("id").unwrap().value().parse::<String>().unwrap();
    let i = query_sid(&var).await;
    ProfileData {username: i[0].username.to_owned() }.to_str().unwrap()
}

#[launch]
async fn rocket() -> _ {
    database::init().await;
    rocket::build().mount(
        "/",
        routes![index, static_files, login, register, logout, secret, profile],
    )
}
