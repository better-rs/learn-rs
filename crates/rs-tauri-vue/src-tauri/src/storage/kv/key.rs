// 存储 key 定义:
pub enum AppStorageKeys {
    AppName,
    AppVersion,
    AppLocale,
}

impl AppStorageKeys {
    pub fn parse(&self) -> &str {
        match self {
            AppStorageKeys::AppLocale => "app:locale",
            AppStorageKeys::AppVersion => "app:version",
            AppStorageKeys::AppName => "app:name",
            _ => "not:found",
        }
    }
}
