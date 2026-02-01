use crate::{handlers::explanation_hu, state::app_state::AppState};

/// 创建解析相关的路由，专门用来管理与原文解析相关的操作
pub fn create_explanation_hu_router() -> axum::Router<AppState> {
    axum::Router::new().route(
        "/query/info/id/{id}",
        axum::routing::get(explanation_hu::query::query_explanation_by_id_handler),
    )
}
