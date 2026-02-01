/// 定义查询条文的 id 范围
#[derive(Debug, serde::Deserialize, Clone, validator::Validate)]
pub struct QueryTreatiseIdParam {
    #[validate(range(min = 1, max = 398, message = "查询条文的 id 必须在 1-398 之间"))]
    pub id: i32,
}
