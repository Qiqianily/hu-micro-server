use hu_server::{
    client::manager::SimpleGrpcClientPool, pb::explanation::GetExplanationByIdRequest,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let max_pool_size = 10;
    let pool = SimpleGrpcClientPool::new("http://[::1]:50001", max_pool_size)?;
    for id in 1..10 {
        let explanation_request = GetExplanationByIdRequest { id };
        let mut client = pool.get_client().await?;
        let response = client.get_explanation_by_id(explanation_request).await?;
        println!(
            "Response: {}",
            serde_json::to_string_pretty(&response.into_inner())?
        );
    }
    Ok(())
}
