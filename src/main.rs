use std::process::Command;

// take 4 arguments from the command line
fn main() {
    // take github_url, branch, commit, test script path as arguments
    let args: Vec<String> = std::env::args().collect();
    let github_url = &args[1];
    let branch = &args[2];
    let commit = &args[3];
    let test_script = &args[4];

    // create a new directory to store the files
    let dir = tempfile::Builder::new()
        .prefix("rust-ci-")
        .tempdir()
        .unwrap();

    // clone the repository
    let repo = git2::Repository::clone(github_url, dir.path()).unwrap();

    // checkout the branch
    let branch = repo.find_branch(branch, git2::BranchType::Local).unwrap();
    repo.set_head(&branch.name().unwrap().unwrap()).unwrap();

    // change the string commit to oid
    let oid = git2::Oid::from_str(commit).unwrap();

    // checkout the oid commit
    let commit = repo.find_commit(oid).unwrap();
    repo.reset(&commit.as_object(), git2::ResetType::Hard, None).unwrap();

    // run the test script and checkout the parent commit if test fails
    // until the test passes or we reach the root commit
    let mut current_commit = commit;
    loop {
        let mut child = Command::new(test_script)
            .current_dir(dir.path())
            .spawn()
            .unwrap();
        let status = child.wait().unwrap();
        if status.success() {
            break;
        }
        // find parent commit
        let parent = current_commit.parent(0).unwrap();
        repo.reset(&parent.as_object(), git2::ResetType::Hard, None).unwrap();
        current_commit = parent;
    }

    // print the current commit id
    println!("{}", current_commit.id());
}