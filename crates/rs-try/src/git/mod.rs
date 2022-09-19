use git2::{Error, MergeAnalysis, MergePreference, Repository};
use std::{fs, path::Path};

pub fn rm_dir(dir: &Path) {
    if is_path_exist(dir) {
        match fs::remove_dir_all(dir) {
            Ok(_) => {},
            Err(err) => {
                panic!("remove dir failed: {:?}, error: {}", dir, err);
            },
        }
    }
}

pub fn is_path_exist(p: &Path) -> bool {
    Path::new(p).exists()
}

pub fn git_clone(url: &str, path: &Path) -> Repository {
    // git clone:
    let repo = match Repository::clone(url, path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
    return repo
}

// ref: https://stackoverflow.com/questions/58768910/how-to-perform-git-pull-with-the-rust-git2-crate
pub fn git_pull(path: &Path, branch: Option<&str>) -> Result<(), Error> {
    let repo = Repository::open(path)?;

    // 分支:
    repo.find_remote("origin")?.fetch(&[branch.unwrap_or("main")], None, None)?;

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    let analysis = repo.merge_analysis(&[&fetch_commit])?;

    if analysis.0.is_up_to_date() {
        Ok(())
    } else if analysis.0.is_fast_forward() {
        let ref_name = format!("refs/heads/{}", branch.unwrap_or("main"));
        let mut reference = repo.find_reference(&ref_name)?;
        reference.set_target(fetch_commit.id(), "Fast-Forward")?;
        repo.set_head(&ref_name)?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
    } else {
        Err(Error::from_str("Fast-forward only!"))
    }
}

pub fn git_sync(url: &str, path: &Path) {
    if is_path_exist(path) {
        println!("repo is exists, do git pull!");
        git_pull(path, None).expect("TODO: panic message");
    } else {
        print!("repo is not exists, do git clone!");
        git_clone(url, path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_sync() {
        let url = "https://github.com/better-rs/.github.git";

        // git clone 结果路径
        let to = "tmp/demo";

        println!("git sync test");
        git_sync(url, to.as_ref());
    }

    #[test]
    fn test_is_path_exist() {
        assert_eq!(is_path_exist("tmp".as_ref()), true);
        assert_eq!(is_path_exist("tmp/2".as_ref()), false);
    }

    #[test]
    fn test_rm_dir() {
        rm_dir("tmp/".as_ref())
    }

    #[test]
    fn test_git_clone() {
        let url = "https://github.com/better-rs/.github.git";

        // git clone 结果路径
        let to = "tmp/demo";

        let repo = git_clone(url, to.as_ref());
    }
}
