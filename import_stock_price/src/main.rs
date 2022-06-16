mod entity;
mod reader;
mod repository;
use reader::{Reader, SFTPCSVReader};
use repository::{PostgresRepository, Repository};
use std::env::{self, VarError};

#[tokio::main]
async fn main() -> Result<(), String> {
    let envs = get_env_settings();

    if envs.iter().any(|x| x.is_err()) {
        return Err(create_error_messages(&envs));
    }

    let [host, username, password, base_dir, db_server, db_userid, db_name, db_port, db_password] =
        envs.map(|x| x.unwrap());

    let filename = "PriceExp_2000_2020.csv".to_owned();
    let reader = SFTPCSVReader::new(host, username, password, base_dir, filename)?;
    let stock_prices = reader.read()?;

    let mut repository =
        PostgresRepository::new(db_server, db_port, db_name, db_userid, db_password).await?;
    repository.insert(stock_prices).await
}

fn create_error_messages(envs: &[Result<String, (VarError, String)>; 9]) -> String {
    envs.iter()
        .filter(|x| x.is_err())
        .map(|x| {
            let (err, key) = x.as_ref().unwrap_err();
            err.to_string() + "(" + key + ")"
        })
        .collect::<Vec<String>>()
        .join(",\n")
}

fn get_env_settings() -> [Result<String, (VarError, String)>; 9] {
    [
        "FILESTORAGE_HOST",
        "FILESTORAGE_USERID",
        "FILESTORAGE_PASSWORD",
        "FILESTORAGE_BASEDIR",
        "DB_SERVERNAME",
        "DB_USERID",
        "DB_NAME",
        "DB_PORT",
        "DB_PASSWORD",
    ]
    .map(|key| env::var(key).map_err(|err| (err, key.to_owned())))
}
