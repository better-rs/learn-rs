// 存储 key 定义:
pub enum AppStorageKeys {
    AppName,
    AppVersion,
    // 语言:
    AppLocale,

    // 显示币种
    SelectedCoins,
    // 基准货币类型:
    BaseCurrency,
    // 查询周期:
    Tick,

    // 交易所 api key:
    ExchangeApiKey,
}

impl AppStorageKeys {
    pub fn parse(&self) -> &str {
        match self {
            AppStorageKeys::AppLocale => "app:locale",
            AppStorageKeys::AppVersion => "app:version",
            AppStorageKeys::AppName => "app:name",
            AppStorageKeys::SelectedCoins => "app:selected:coins",
            AppStorageKeys::BaseCurrency => "app:base:currency",
            AppStorageKeys::Tick => "app:tick",
            AppStorageKeys::ExchangeApiKey => "app:exchange",

            _ => "not:found",
        }
    }
}
