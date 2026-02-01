use std::time::Duration;

use tonic::transport::{Channel, Endpoint};

use crate::{
    pb::explanation::explanation_hu_service_client::ExplanationHuServiceClient,
    response::{ApiResult, errors::ApiError},
};

/// 简单的连接池实现
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct SimpleGrpcClientPool {
    endpoint: Endpoint,
    max_pool_size: usize,
}

impl SimpleGrpcClientPool {
    pub fn new(addr: &str, max_pool_size: usize) -> anyhow::Result<Self> {
        let endpoint = Endpoint::from_shared(addr.to_string())?
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(5))
            .keep_alive_while_idle(true);

        Ok(Self {
            endpoint,
            max_pool_size,
        })
    }

    // 创建 Channel（tonic 会自动复用连接）
    pub async fn get_channel(&self) -> ApiResult<Channel> {
        self.endpoint
            .connect()
            .await
            .map_err(|e| ApiError::Internal(e.into()))
    }

    // 获取客户端
    pub async fn get_client(&self) -> ApiResult<ExplanationHuServiceClient<Channel>> {
        let channel = self.get_channel().await?;
        Ok(ExplanationHuServiceClient::new(channel))
    }
}
