// 加密kv存储方案:
pub struct AppEncryptedKVStorage {}

impl AppEncryptedKVStorage {
    pub fn default() -> AppEncryptedKVStorage {
        Self {}
    }

    pub fn get_locale(&self) -> Option<String> {
        Some("en".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let kv = AppEncryptedKVStorage::default();

        let ret = kv.get_locale();
        println!("{:?}", ret);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
