use git2::Repository;

fn main() {
    let url = "https://github.com/better-rs/.github.git";

    // git clone 结果路径
    let to = "tmp/demo";

    // git clone:
    let repo = match Repository::clone(url, to) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    println!("hello git!");
}
