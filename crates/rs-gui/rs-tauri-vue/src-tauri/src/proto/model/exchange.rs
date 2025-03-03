use serde::{Deserialize, Serialize};

// 交易所参数:
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExchangeEntity {
    pub exchange_name: String,

    // keys:
    pub keys: Vec<ApiKeyEntity>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiKeyEntity {
    // api key:
    pub key: String,

    // 类型: read, write
    pub typ: String,

    // 备注:
    pub label: String,

    // 激活状态:
    pub is_activated: bool,
}
