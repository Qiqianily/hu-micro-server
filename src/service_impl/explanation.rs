use std::{ops::Deref, sync::Arc};

use crate::{
    pb::explanation::{
        GetExplanationByIdRequest, GetExplanationByIdResponse,
        explanation_hu_service_server::ExplanationHuService,
    },
    utils::{convert::datetime_to_timestamp, serializer_items::deserialize_explanation_items},
};
use sqlx::{
    PgPool,
    types::chrono::{DateTime, Utc},
};

use tonic::{Request, Response, Status};
use uuid::Uuid;

/// 内部数据状态
#[derive(Debug, Clone)]
pub struct AppStateInner {
    pub pool: &'static PgPool,
}
/// 实现解引用操作
impl Deref for ExplanationHuServiceImpl {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// 实现 GreeterService trait
#[derive(Debug)]
pub struct ExplanationHuServiceImpl {
    // 这里面可以放数据库连接池
    pub inner: Arc<AppStateInner>,
}
impl ExplanationHuServiceImpl {
    pub fn new(pool: &'static PgPool) -> Self {
        Self {
            inner: Arc::new(AppStateInner { pool }),
        }
    }
}
#[tonic::async_trait]
impl ExplanationHuService for ExplanationHuServiceImpl {
    async fn get_explanation_by_id(
        &self,
        request: Request<GetExplanationByIdRequest>,
    ) -> Result<Response<GetExplanationByIdResponse>, Status> {
        let id = request.into_inner().id;
        // 查询数据库
        let row: (i32, Uuid, serde_json::Value, Vec<String>, DateTime<Utc>)  = sqlx::query_as(
                "SELECT treatise_id as id, uuid, explanation, summary, created_at FROM explain_hu WHERE treatise_id = $1",
            )
            .bind(id)
            .fetch_one(self.inner.pool)
            .await.map_err(|_e| Status::not_found("Data not found"))?;
        let explanation_items = deserialize_explanation_items(row.2).unwrap();
        let created_at = datetime_to_timestamp(row.4);
        let result = GetExplanationByIdResponse {
            treatise_id: row.0,
            uuid: row.1.to_string(),
            explanation_items,
            summary: row.3,
            created_at: Some(created_at),
        };
        // 返回结果
        Ok(Response::new(result))
    }
}
