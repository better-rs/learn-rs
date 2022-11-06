// kv存储方案:
pub struct AppKvStorage {}

impl AppKvStorage {
    pub fn default() -> AppKvStorage {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
