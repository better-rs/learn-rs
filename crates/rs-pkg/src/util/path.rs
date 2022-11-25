use std::{env, fs, path::PathBuf};

pub fn local_dir(folder: Option<&str>) -> PathBuf {
    let current_dir = env::current_dir().expect("get current directory failed");

    let dest_dir = current_dir.join(folder.unwrap_or("tmp"));
    if !dest_dir.exists() {
        fs::create_dir(&dest_dir).expect("create dir failed");
    }
    dest_dir
}

pub fn local_sqlite_url(db_name: Option<&str>) -> &str {
    let fp = db_name.unwrap_or("app.db");
    let prefix = "sqlite:";

    // get or create:
    let d = local_dir(None);

    // return db url
    let db_url = format!("{}{}", prefix, d.join(fp).to_str().unwrap());
    Box::leak(db_url.into_boxed_str())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_local_sqlite_url() {
        let cases = [None, Some("test.db")];

        // iter
        for case in cases.iter() {
            let url = local_sqlite_url(*case);
            println!("url: {}", url);
        }
    }
}
