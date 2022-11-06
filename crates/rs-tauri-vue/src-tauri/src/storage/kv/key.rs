// 存储 key 定义:
pub enum AppStorageKeys {
    AppLocale,
    AppVersion,
}

impl AppStorageKeys {
    pub fn parse(&self) -> &str {
        match self {
            AppStorageKeys::AppLocale => "app:locale",
            AppStorageKeys::AppVersion => "app:version",
        }
    }
}
