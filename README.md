# webdev_in_rust
 - Backend build with [rocket.rs](https://rocket.rs/) and frontend build with [yew.rs](https://yew.rs/). For the database i'm using [SurrealDB](https://surrealdb.com/).
 - I followed the OWASP cheat sheets for implementing the user [authentication](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html) and [password storage](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html). 
   - (some features are still missing, e.g. password changing or password resetting is not implemented yet)
   - Passwords are stored hashed, salted and peppered and password requirements are what OWASP suggests.
 - [Password strength estimator](https://github.com/shssoichiro/zxcvbn-rs) on the frontend and backend uses [haveibeenpwned.com api](https://github.com/wisespace-io/pwned-rs) to check for known passwords.
