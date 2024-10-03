use std::{fs, path::Path, sync::Arc, time::Duration};

use anyhow::Context;
use once_cell::sync::Lazy;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::runtime::Runtime;
use tracing::log::LevelFilter;

use crate::{config::get, error::AppResult, utils::convert::Convert};

pub static DBCONN: Lazy<Arc<DatabaseConnection>> = Lazy::new(|| {
    let rt = Runtime::new().unwrap();
    rt.block_on(async { Arc::new(db_init().await.expect("Failed to initialize database")) })
});

async fn db_init() -> AppResult<DatabaseConnection> {
    let url = get_url().await?;

    // 创建相关目录
    if get("database_driver").unwrap_or("sqlite".to_string()) == "sqlite" {
        ensure_data_directory()?;
    };

    // 设置连接选项
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(6))
        .acquire_timeout(Duration::from_secs(6))
        .idle_timeout(Duration::from_secs(6))
        .max_lifetime(Duration::from_secs(6))
        .sqlx_logging(get("database_log_enable")?.to_bool()?)
        .sqlx_logging_level(LevelFilter::Info);

    // 初始化连接
    let db = Database::connect(opt).await?;

    Ok(db)
}

//==================================================================================
//= 确保sqlite数据库目录存在
fn ensure_data_directory() -> AppResult<()> {
    let binding = get("database_sqlite_dbfile")?;
    let path = Path::new(binding.as_str()).parent();

    if let Some(path) = path {
        tracing::info!("ensure data directory exists");
        fs::create_dir_all(path).context("create data directory failed")?;
    };

    Ok(())
}

//==================================================================================
//= 获取数据库连接地址
async fn get_url() -> AppResult<String> {
    match get("database_driver")
        .unwrap_or("sqlite".to_string())
        .as_str()
    {
        "mysql" => Ok(format!(
            "mysql://{}:{}@{}:{}/{}?parseTime=true",
            get("database_username")?,
            get("database_password")?,
            get("database_host")?,
            get("database_port")?,
            get("database_dbname")?
        )),
        "postgres" => Ok(format!(
            "postgres://{}:{}@{}:{}/{}?currentSchema=my_schema",
            get("database_username")?,
            get("database_password")?,
            get("database_host")?,
            get("database_port")?,
            get("database_dbname")?
        )),
        "sqlite" => Ok(format!(
            "sqlite://{}?mode=rwc",
            get("database_sqlite_dbfile").unwrap_or("data/data.db".to_string())
        )),
        _ => panic!("Failed to construct database URL"),
    }
}
