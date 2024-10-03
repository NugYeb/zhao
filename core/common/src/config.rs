use std::env;

use anyhow::Context;
use colored::Colorize;
use tracing::info;

use crate::error::AppResult;

//==================================================================================
//= 加载环境变量
pub fn load_env() -> AppResult<()> {
    dotenvy::dotenv().context("context")?;
    println!(
        "{}",
        "\tSucceed to load environment variables!"
            .bright_green()
            .bold()
    );
    info!("Succeed to load environment variables!");
    Ok(())
}

pub fn get(str: &str) -> AppResult<String> {
    Ok(env::var(str)?)
}
