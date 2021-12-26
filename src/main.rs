use std::{process::Command, env::{current_dir, self}};

fn main() {
    // take github_url, branch, commit, test script path as arguments
    let args: Vec<String> = std::env::args().collect();
    let github_url;
    let branch;
    let commit;
    let test_script;
    // exit if not enough arguments
    if args.len() < 4 {
        println!("Usage: salmon <github_url> <branch> <commit> <test_script_path>");
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

    // print arguments
    println!("github_url: {}", github_url);
    println!("branch: {}", branch);
    println!("commit: {}", commit);
    println!("test_script: {}", test_script);

    // setup directory
    let dir = format!("salmon-{}-{}", "Salmon-Test",branch);
    println!("dir: {}", dir);
    let dir_path = std::path::Path::new(&dir);
    if dir_path.exists() {
        std::fs::remove_dir_all(&dir).unwrap();
    }
    std::fs::create_dir(&dir).unwrap();

    // clone the repository
    let repo = git2::Repository::clone(github_url, dir_path)
        .expect(format!("Failed to clone repository : {}", github_url).as_str());
    println!("Cloned repository {} into {}", github_url.clone(), dir_path.to_string_lossy());

    // checkout the branch
    // println!("Current head: {}", repo.head().unwrap().name().unwrap());
    // let branch = repo.find_branch(branch, git2::BranchType::Local)
    //     .expect( format!("Failed to find branch, branch name : {}", branch).as_str());
    // let branch_name = &branch.name().unwrap().unwrap();
    // println!("Found branch : {}", branch_name.clone());

    let (object, reference) = repo.revparse_ext(commit).expect("Object not found");
    repo.checkout_tree(&object, None).expect("Failed to checkout tree");
    match reference {
        Some(ref_) => {
            repo.set_head(ref_.name().unwrap()).expect("Failed to set head");
        },
        None => {
            repo.set_head_detached(object.id()).expect("Failed to set head");
        },
    }
    
    // change the string commit to oid
    let oid = git2::Oid::from_str(commit).unwrap();
    let commit = repo.find_commit(oid).unwrap();

    // run the test script and checkout the parent commit if test fails
    // until the test passes or we reach the root commit
    let mut current_commit = commit;
    loop {
        let test_script_with_path = format!("{}/{}", env::current_dir().unwrap().to_str().unwrap(), test_script);
        let mut child = Command::new(test_script_with_path)
            .current_dir(dir_path)
            .spawn()
            .unwrap();
        let status = child.wait().unwrap();
        if status.success() {
            println!("Test passed on commit : {}", current_commit.id());
            break;
        }
        // find parent commit
        let parent = current_commit.parent(0).unwrap();
        repo.reset(&parent.as_object(), git2::ResetType::Hard, None)
            .unwrap();
        current_commit = parent;
    }

    
}
