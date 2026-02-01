/// 服务端口和日志信息相关配置
#[derive(Debug, serde::Deserialize)]
pub struct GrpcConfig {
    port: u16,
    log_level: String,
}

impl GrpcConfig {
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
}
