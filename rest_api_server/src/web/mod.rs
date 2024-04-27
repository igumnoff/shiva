pub mod mw_auth; //Подключает файл mw_auth.rs
pub mod routes_login; //Подключает файл routes_login.rs
pub mod routes_tickets; //Подключает файл routes_tickets.rs
pub const AUTH_TOKEN: &str = "auth_token"; //Задаем токен аутентификации (в реальном приложении необходимо воспользоваться другими методами создания токена!!!!)
