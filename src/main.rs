use std::{process::Command};

fn main() {
    // take github_url, branch, commit, test script path as arguments
    let args: Vec<String> = std::env::args().collect();
    let github_url ;
    let branch ;
    let commit ;
    let test_script;
    // exit if not enough arguments
    if args.len() < 4 {
        println!("Usage: cargo run <github_url> <branch> <commit> <test_script_path>");
        std::process::exit(1);
    } else {
        // assign arguments to variables
        github_url = &args[1];
        branch = &args[2];
        commit = &args[3];
        // test script path is optional
        if args.len() == 4 {
            // if no test script path is given, use default
            test_script = "test.sh".to_string();
        } else {
            // if test script path is given, use it
            test_script = args[4].clone();    
        }   
    }
    
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
        let mut child = Command::new(test_script.clone())
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