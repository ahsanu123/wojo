use std::path::Path;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::OnceCell;

static MAX_CONNECTION: u32 = 100;
static MIN_CONNECTION: u32 = 3;

pub static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db_connection() -> &'static DatabaseConnection {
    let current_dir = std::env::current_dir().expect("fail to get current directory");
    let sql_path = Path::join(&current_dir, "wojo.sqlite")
        .into_os_string()
        .into_string()
        .expect("invalid sql path");

    let mut connect_option = ConnectOptions::new(sql_path);

    connect_option
        .max_connections(MAX_CONNECTION)
        .min_connections(MIN_CONNECTION)
        .sqlx_logging(true);

    DATABASE_CONNECTION
        .get_or_init(|| async {
            Database::connect(connect_option)
                .await
                .expect("fail to create connection")
        })
        .await
}
