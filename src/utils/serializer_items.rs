use serde::{Deserialize, Deserializer, Serializer, de, ser::SerializeSeq};
// 自定义序列化：将 Vec<ExplanationItem> 序列化为 JSON 数组
pub fn serialize_explanation_items<S>(
    items: &Vec<crate::pb::explanation::ExplanationItem>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // 如果 items 为空，序列化为空数组
    if items.is_empty() {
        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    } else {
        // 创建自定义序列化适配器
        let mut seq = serializer.serialize_seq(Some(items.len()))?;

        for item in items {
            // 将每个 ExplanationItem 序列化为单个键值对的对象
            let mut map = serde_json::Map::new();
            map.insert(
                item.key.clone(),
                serde_json::Value::String(item.value.clone()),
            );
            seq.serialize_element(&map)?;
        }

        seq.end()
    }
}

// 自定义反序列化：将 JSON 数组反序列化为 Vec<ExplanationItem>
pub fn deserialize_explanation_items<'de, D>(
    deserializer: D,
) -> Result<Vec<crate::pb::explanation::ExplanationItem>, D::Error>
where
    D: Deserializer<'de>,
{
    // 先反序列化为 Value
    let value = serde_json::Value::deserialize(deserializer)?;

    match value {
        serde_json::Value::Array(arr) => {
            let mut items = Vec::new();

            for item in arr {
                if let serde_json::Value::Object(map) = item {
                    // 每个对象应该只有一个键值对
                    if let Some((key, value)) = map.into_iter().next() {
                        let value_str = match value {
                            serde_json::Value::String(s) => s,
                            serde_json::Value::Number(n) => n.to_string(),
                            serde_json::Value::Bool(b) => b.to_string(),
                            serde_json::Value::Null => "null".to_string(),
                            _ => {
                                // 对于数组或嵌套对象，序列化为 JSON 字符串
                                serde_json::to_string(&value).unwrap_or_else(|_| "".to_string())
                            }
                        };

                        items.push(crate::pb::explanation::ExplanationItem {
                            key,
                            value: value_str,
                        });
                    }
                }
            }

            Ok(items)
        }
        serde_json::Value::Null => Ok(Vec::new()),
        _ => Err(de::Error::custom(
            "explanation_items must be an array or null",
        )),
    }
}
