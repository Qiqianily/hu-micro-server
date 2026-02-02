use std::{ops::Deref, sync::Arc};

use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::{
    pb::explanation::explanation_hu_service_client::ExplanationHuServiceClient, response::ApiResult,
};

// AppStateInner is a struct that holds the inner state of the application.
// It is used to store application-specific data that needs to be shared between different parts of the application.
#[derive(Debug, Clone)]
pub struct AppStateInner {}

/// AppState app 的状态
///
/// # 成员
/// - grpc_client: redis client 里面维持了一个连接池
/// - inner: 内部共享状态
// AppState is a struct that holds the state of the application.
// It contains a pool of postgres connections and a reference to an inner struct.
#[derive(Debug, Clone)]
pub struct AppState {
    pub grpc_client: Arc<Mutex<ExplanationHuServiceClient<Channel>>>,
    pub inner: Arc<AppStateInner>,
}
// construct a new AppState object with a pool of postgres connections and an inner struct.
impl AppState {
    pub async fn new(grpc_addr: &str) -> ApiResult<Self> {
        // 获取全局的静态 database pool 引用
        // let grpc_client = get_global_grpc_client_pool();
        // 创建 gRPC 客户端
        let channel = tonic::transport::Channel::from_shared(grpc_addr.to_string())
            .expect("创建 EndPoint 时出错！")
            .connect()
            .await
            .expect("连接 GRPC 时出错了！");
        let grpc_client = Arc::new(Mutex::new(ExplanationHuServiceClient::new(channel)));
        Ok(Self {
            grpc_client,
            inner: Arc::new(AppStateInner {}),
        })
    }
}

// Deref allows us to access the inner struct of an AppState object using the dot operator.
// This is useful for accessing application-specific data stored in the inner struct.
impl Deref for AppState {
    type Target = AppStateInner; // the type of the inner struct
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
