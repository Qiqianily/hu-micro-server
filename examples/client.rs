use hu_server::pb::explanation::{
    GetExplanationByIdRequest, explanation_hu_service_client::ExplanationHuServiceClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut explanation_client = ExplanationHuServiceClient::connect("http://[::1]:50001").await?;
    let explanation_request = GetExplanationByIdRequest { id: 27 };
    let explanation_response = explanation_client
        .get_explanation_by_id(explanation_request)
        .await?;
    println!("response: {:?}", explanation_response);
    let r = explanation_response.into_inner();
    println!("✅ Explanation response:");
    println!("Answer: {}", serde_json::to_string_pretty(&r)?);
    Ok(())
}
