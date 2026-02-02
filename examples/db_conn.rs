use hu_server::{
    conf::app::AppConfig, db::pgsql::init_database_pool_with_config,
    log::logger::init_logger_with_file,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::load()?;
    eprintln!("config:{:?}", config);
    let log_level = config.grpc_config().log_level();
    let _guard = init_logger_with_file(log_level).await?;
    init_database_pool_with_config(config.database()).await?;
    Ok(())
}
