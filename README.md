# Salmon

Like salmon leaping up the fall, this program goes up the git log until the test succeed.  

Nowadays, there are many tools to test every code before and after commit to git. But when you forgot to add test for specific feature, it's hard to know when the feature broken for the first time. 

Salmon is a tool to help you find the start of the bug by automating checkout and test.

## How it works

This salmon takes four arguments : git url, branch, commit, test script name. Salmon clones git repository with given branch, and checkout the given commit. Then it makes test with given test script name. When the test fails, salmon goes up to its parent commit and test again. It will be repeated until test succeed

Before running salmon, you should make sure the test script is in the same directory as 'salmon.sh'. and 'salmon.sh' must exit with non-zero exit code when test failed.

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
Given 
- git repository which has commit history, and there is a test failed at recent commit.
- test script named 'salmon.sh', which exits with non-zero exit code when test failed.

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
Test failed on commit: 9586101ce7297a4fc39e7a0f62e96806ed3d7c7a with result : 1  
Test passed on commit: cbba9813123a5ee6b9776462539df1604a6b5566  

commit hash | content | test result | note
:----------:|:-------:| :--------:|:---:
d07f83fc3e83bcbc1750fd6782fa5faeaa75cdc8 | d | 1 | HEAD
9586101ce7297a4fc39e7a0f62e96806ed3d7c7a | c | 1 | START
cbba9813123a5ee6b9776462539df1604a6b5566 | b | 0 | END
600da9654eb364862b4a6bc65861fdc56000a3ef | a | 1 |

You can test with other commit hash and test script.
``` bash
# with other commit hash
cargo run https://github.com/roquen4145/Salmon-Test test d07f83fc3e83bcbc1750fd6782fa5faeaa75cdc8
# or with other test script
cargo run https://github.com/roquen4145/Salmon-Test test 9586101ce7297a4fc39e7a0f62e96806ed3d7c7a anohter_salmon.sh
# or with other branch
cargo run https://github.com/roquen4145/Salmon-Test main 9586101ce7297a4fc39e7a0f62e96806ed3d7c7a
```