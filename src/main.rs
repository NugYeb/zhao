use api::router::init::init_router;
use colored::Colorize;
use common::{config, error::AppResult, log};
use extend::core_server::start_server;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> AppResult<()> {
    println!("{}", "\n\tStarting...".bright_green().bold());

    //==================================================================================
    //= 初始化配置、环境变量
    //= init config or env
    config::load_env()?;

    //==================================================================================
    //= 初始化日志
    //= init logger
    let _guard = log::init_logger()?;

    //==================================================================================
    //= 启动插件rpc服务监听
    //= start plugin rpc server listener
    start_server().await?;

    //==================================================================================
    //= 初始化路由，启动web服务
    //= init router and start web server
    init_router().await;

    Ok(())
}
