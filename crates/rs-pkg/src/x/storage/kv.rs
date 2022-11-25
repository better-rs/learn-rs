use std::{io::Write, path::PathBuf};

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

use crate::util;

// kv存储方案:
pub struct KvClient {
    pub cli: PickleDb,
}

impl KvClient {
    pub fn default() -> Self {
        Self::new(None, None)
    }

    pub fn new(
        db_path: Option<PathBuf>,
        serialization_method: Option<SerializationMethod>,
    ) -> Self {
        let db_name = "app.kv.json";
        let fp = db_path.unwrap_or(util::local_dir(None).join(db_name));
        // println!("kv file: {:?}", fp);

        // default = json
        let m = serialization_method.unwrap_or(SerializationMethod::Json);

        // load file:
        if fp.exists() {
            let db = PickleDb::load(fp.clone(), PickleDbDumpPolicy::AutoDump, m)
                .expect("load kv db failed");
            return Self { cli: db }
        }

        // new file:
        let db = PickleDb::new(fp, PickleDbDumpPolicy::AutoDump, m);
        Self { cli: db }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_kv_client() {
        let mut kv = KvClient::default();

        let case = [
            ("test", "test value"),
            ("test2", "test value2"),
            ("test3", "test value3"),
            ("test4", "test value4"),
        ];

        for (k, v) in case.iter() {
            kv.cli.set(k, v);

            let v: String = kv.cli.get(k).unwrap();
            print!("k={}, v={}", k, v);
        }
    }
}
