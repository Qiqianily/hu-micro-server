fn main() -> anyhow::Result<()> {
    // 如果目录不存在则创建
    std::fs::create_dir_all("src/pb")?;
    let build = tonic_prost_build::configure()
        .type_attribute(
            "explanation.ExplanationItem",
            r#"
        #[derive(
            serde::Serialize,
            serde::Deserialize
        )]
        #[serde(rename_all = "camelCase")]
        "#,
        )
        .type_attribute(
            "explanation.GetExplanationByIdResponse",
            r#"
        #[derive(
            serde::Serialize,
            serde::Deserialize
        )]
        #[serde(rename_all = "camelCase")]
        "#,
        )
        .type_attribute(
            "explanation.GetExplanationByIdRequest",
            r#"
        #[derive(
            serde::Serialize,
            serde::Deserialize,
            sqlx::FromRow
        )]
        #[serde(rename_all = "camelCase")]
        "#,
        ).field_attribute(
            "explanation.GetExplanationByIdResponse.explanation_items",
            r#"
                #[serde(
                    serialize_with = "crate::utils::serializer_items::serialize_explanation_items",
                    deserialize_with = "crate::utils::serializer_items::deserialize_explanation_items",
                    skip_serializing_if = "Vec::is_empty",
                    default
                )]
                "#,
        ).field_attribute(
            "explanation.GetExplanationByIdResponse.created_at",
            r#"
            #[serde(with = "crate::utils::time_serializer")]
            "#,
        );
    build
        .out_dir("src/pb")
        .compile_protos(&["proto/explanation/explanation.proto"], &["proto"])?;
    Ok(())
}
