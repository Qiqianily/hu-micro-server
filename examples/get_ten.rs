use hu_server::pb::explanation::{
    GetExplanationByIdRequest, explanation_hu_service_client::ExplanationHuServiceClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = ExplanationHuServiceClient::connect("http://[::1]:50001").await?;
    for id in 1..10 {
        let explanation_request = GetExplanationByIdRequest { id };
        let response = client.get_explanation_by_id(explanation_request).await?;
        println!(
            "Response: {}",
            serde_json::to_string_pretty(&response.into_inner())?
        );
    }
    Ok(())
}
