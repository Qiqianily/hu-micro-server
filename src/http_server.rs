use hu_server::{app, conf, log};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 读取配置信息
    let config = conf::get_app_config();
    let grpc_addr = format!("hu_server_grpc:{}", config.grpc_config().port());
    println!("grpc_addr: {}", grpc_addr);
    // 2. 初始化日志，为了防止多线程日志写入不完整，要保留 guard，main 函数结束时释放
    let log_level = config.http_config().log_level();
    if config.is_log_file() {
        let _guard = log::logger::init_logger_with_file(log_level).await?;
    } else {
        log::logger::init_logger_without_file(log_level).await?;
    }

    // 3. 启动服务
    app::server::Server::new(config)
        .start_server(&grpc_addr)
        .await?;

    Ok(())
}
