use axum::{debug_handler, extract::State};

use crate::{
    common::valid::ValidPath,
    handlers::common::model::QueryTreatiseIdParam,
    pb::explanation::{GetExplanationByIdRequest, GetExplanationByIdResponse},
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::app_state::AppState,
};

/// 按 id 来查询
#[debug_handler]
pub async fn query_explanation_by_id_handler(
    State(AppState { grpc_client, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidPath(params): ValidPath<QueryTreatiseIdParam>,
) -> ApiResult<ApiResponse<GetExplanationByIdResponse>> {
    let id = params.id;
    let explanation_request = GetExplanationByIdRequest { id };
    let mut client = grpc_client.get_client().await?;
    let grpc_response = match client.get_explanation_by_id(explanation_request).await {
        Ok(response) => response.into_inner(),
        Err(status) => {
            return Err(ApiError::Internal(status.into()));
        }
    };
    Ok(ApiResponse::success(grpc_response))
}
