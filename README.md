# Salmon

Like salmon leaping up the fall, this program goes up the git log until the test succeed.  
You can easily find the start point of test failure by using salmon.

## How it works

This salmon takes four arguments : git url, branch, commit, test script path. Salmon clones git repository with given branch, and checkout the given commit. Then it makes test with given test script name. When the test fails, salmon goes up to its parent commit and test again. It will be repeated until test succeed

## How to build & test
``` bash
cargo run <github url> <branch name> <commit hash> <test script name = "salmon.sh">
```
After get binary, you can run it with 
``` bash
./salmon <github url> <branch name> <commit hash> <test script name = "salmon.sh">
```
If you edited 'salmon.sh' script, you can also run it without script name.


## Example
``` bash
# run with default test script
cargo run https://github.com/roquen4145/Salmon-Test test 9586101ce7297a4fc39e7a0f62e96806ed3d7c7a
```
### result
SalmonConfig {
    github_url: "https://github.com/roquen4145/Salmon-Test",  
    branch: "test",  
    commit: "9586101ce7297a4fc39e7a0f62e96806ed3d7c7a",  
    test_script: "salmon.sh",  
}  
Test directory: salmon-Salmon-Test-test  
Cloned repository https://github.com/roquen4145/Salmon-Test into salmon-Salmon-Test-test  
Start test from commit 9586101ce7297a4fc39e7a0f62e96806ed3d7c7a  
Test failed on commit: 9586101ce7297a4fc39e7a0f62e96806ed3d7c7a with result :   exit code: 1
Test passed on commit: cbba9813123a5ee6b9776462539df1604a6b5566  
