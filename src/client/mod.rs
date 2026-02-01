use std::sync::OnceLock;

use crate::client::manager::SimpleGrpcClientPool;

pub mod manager;
/// 全局 Grpc Client 连接池实例
static GLOBAL_GRPC_CLIENT_POOL: OnceLock<SimpleGrpcClientPool> = OnceLock::new();
/// 获取全局的静态 Grpc Client 连接池实例
pub fn get_global_grpc_client_pool() -> &'static SimpleGrpcClientPool {
    GLOBAL_GRPC_CLIENT_POOL
        .get()
        .expect("grpc client pool lost")
}
/// 初始化全局的静态 Grpc Client 连接池实例
pub async fn set_global_grpc_client_pool(addr: &str, max_pool_size: usize) -> anyhow::Result<()> {
    let pool = SimpleGrpcClientPool::new(addr, max_pool_size)?;
    GLOBAL_GRPC_CLIENT_POOL
        .set(pool)
        .expect("set grpc client pool failed");
    Ok(())
}
