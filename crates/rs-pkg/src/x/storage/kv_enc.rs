use std::path::PathBuf;

use microkv::MicroKV;
use serde::de::DeserializeOwned;

use crate::util;

// 加密kv存储方案:
pub struct KvEncryptedClient {
    pub cli: MicroKV,
}

impl KvEncryptedClient {
    pub fn default() -> Self {
        Self::new(None, None)
    }

    pub fn new(db_path: Option<PathBuf>, unsafe_pwd: Option<&str>) -> Self {
        let db_name = "app_test";
        let pwd = unsafe_pwd.unwrap_or("unsafe_pwd");

        let db_path = db_path.unwrap_or_else(|| util::local_dir(None).join(db_name));
        let fp = db_path.as_path().to_str().expect("db_path to str failed");

        let cli: MicroKV = MicroKV::open_with_base_path(fp, PathBuf::new())
            .expect("Failed to create MicroKV from a stored file or create MicroKV for this file")
            .set_auto_commit(true)
            .with_pwd_clear(pwd);

        Self { cli }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_kv_enc_client() {
        let mut kv = KvEncryptedClient::default();

        let cases =
            [("k1", "v1"), ("k2", "v2"), ("k3", "v3"), ("k4", "v4"), ("k5", "v5"), ("k6", "v6")];

        for (k, v) in cases.iter() {
            kv.cli.put(k, v).unwrap();
            let vv: String = kv.cli.get_unwrap(k).unwrap();
            println!("{}: {:?}", k, vv);
        }
    }
}
