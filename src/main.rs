use std::{env, path::PathBuf, process::Command};

#[derive(Debug)]
pub struct SalmonConfig {
    pub github_url: String,
    pub branch: String,
    pub commit: String,
    pub test_script: String,
}

fn get_config() -> SalmonConfig {
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
            test_script = "salmon.sh".to_string();
        } else {
            // if test script path is given, use it
            test_script = args[4].clone();
        }
    }

    let salmon_config = SalmonConfig {
        github_url: github_url.to_string(),
        branch: branch.to_string(),
        commit: commit.to_string(),
        test_script: test_script.to_string(),
    };
    println!("{:#?}", salmon_config);
    salmon_config
}

fn setup_test_directory(salmon_config: &SalmonConfig) -> PathBuf {
    let dir = format!(
        "salmon-{}-{}",
        salmon_config
            .github_url
            .clone()
            .rsplit_terminator('/')
            .collect::<Vec<&str>>()[0],
        salmon_config.branch.clone()
    );
    println!("Test directory: {}", dir);
    let dir_path = std::path::Path::new(&dir);
    if dir_path.exists() {
        std::fs::remove_dir_all(&dir).unwrap();
    }
    std::fs::create_dir(&dir).unwrap();

    dir_path.to_path_buf()
}

fn setup_repository(salmon_config: &SalmonConfig, dir_path: PathBuf) {
    // clone the repository
    let repo = git2::Repository::clone(&salmon_config.github_url, &dir_path)
        .expect(format!("Failed to clone repository: {}", salmon_config.github_url).as_str());
    println!(
        "Cloned repository {} into {}",
        salmon_config.github_url,
        &dir_path.to_string_lossy()
    );

    // checkout the branch
    let (object, reference) = repo
        .revparse_ext(&format!("remotes/origin/{}", salmon_config.branch.clone()))
        .expect("Branch not found");
    repo.checkout_tree(&object, None)
        .expect("Failed to checkout tree");
    match reference {
        Some(ref_) => {
            repo.set_head(ref_.name().unwrap())
                .expect("Failed to set branch");
        }
        None => {
            repo.set_head_detached(object.id())
                .expect("Failed to set branch");
        }
    }
}

fn run_test(salmon_config: &SalmonConfig, dir_path: PathBuf) {
    // run the test script and checkout the parent commit if test fails
    // until the test passes or we reach the root commit
    let repo = git2::Repository::open(&dir_path).unwrap();
    let oid = git2::Oid::from_str(&salmon_config.commit).unwrap();
    let mut current_commit = repo.find_commit(oid).unwrap();
    println!("Start test from commit {}", salmon_config.commit);
    loop {
        let test_script_with_path = format!(
            "{}/{}",
            env::current_dir().unwrap().to_str().unwrap(),
            salmon_config.test_script
        );
        let mut child = Command::new(test_script_with_path)
            .current_dir(dir_path.clone())
            .spawn()
            .unwrap();
        let status = child.wait().unwrap();
        if status.success() {
            println!("Test passed on commit: {}", current_commit.id());
            break;
        } else {
            println!(
                "Test failed on commit: {} with result : {}",
                current_commit.id(),
                status.code().unwrap()
            );
        }
        // find parent commit
        let parent = current_commit.parent(0);
        match parent {
            Ok(parent_commit) => {
                repo.reset(&parent_commit.as_object(), git2::ResetType::Hard, None)
                    .unwrap();
                current_commit = parent_commit;
            }
            Err(_) => {
                println!("No parent commit found, test failed");
                break;
            }
        }
    }
}

fn main() {
    let salmon_config = get_config();
    let dir_path = setup_test_directory(&salmon_config);
    setup_repository(&salmon_config, dir_path.clone());
    run_test(&salmon_config, dir_path)
}
