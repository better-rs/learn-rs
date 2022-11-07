use crate::storage::AppStorageKeys;
use pickledb::{
    error::ErrorType::Serialization, PickleDb, PickleDbDumpPolicy, SerializationMethod,
};
use std::{
    env, fs,
    io::Write,
    path,
    path::{Path, PathBuf},
};
use tracing::info;

// kv存储方案:
pub struct AppKvStorage {
    db: PickleDb,
}

impl AppKvStorage {
    pub fn default() -> AppKvStorage {
        let _root_dir = tauri::api::path::document_dir();
        let db_name = "app.test.kv.json";

        let mut current_dir = env::current_dir().expect("get current directory failed");
        let tmp_dir = current_dir.join("tmp");
        let fp = tmp_dir.join(db_name);

        // create temporary directory:
        fs::create_dir_all(tmp_dir.clone()).expect("create dir failed");

        println!("kv storage file: {:?}", fp);
        if !fp.exists() {
            // let mut f = fs::File::create(fp.clone()).expect("create file failed");
            // let content = "{\"app\": \"app\"}";
            // f.write(content.as_ref()).unwrap();

            //
            // // new:
            // let db = PickleDb::new(
            //     tmp_dir.clone(),
            //     PickleDbDumpPolicy::DumpUponRequest,
            //     SerializationMethod::Json,
            // );
            //
            // println!("new storage: {:?}", tmp_dir);
            // return Self { db }
        }

        // load an existing DB from a file (the same file in this case)
        // let db = PickleDb::load(
        //     fp.clone(),
        //     PickleDbDumpPolicy::DumpUponRequest,
        //     SerializationMethod::Json,
        // )
        // .unwrap();

        let db = PickleDb::new(fp, PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json);
        Self { db }
    }

    pub fn from_json(path: PathBuf) -> AppKvStorage {
        // if !path.exists() {
        //     fs::create_dir_all(path.clone()).expect("create dir failed");
        // }

        let db =
            PickleDb::load(path, PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json)
                .unwrap();

        Self { db }
    }

    pub fn set_locale(&mut self, locale: &str) -> pickledb::error::Result<()> {
        // let key = "app:locale";
        let key = AppStorageKeys::AppLocale.parse();

        self.db.set(key, &locale)
    }

    pub fn get_locale(&self) -> String {
        let key = AppStorageKeys::AppLocale.parse();

        self.db.get(key).unwrap_or("en".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut kv = AppKvStorage::default();

        kv.set_locale("fr").expect("TODO: panic message");

        println!("get_locale : {:?}", kv.get_locale());

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
