# Salmon

Like salmon leaping up the fall, this program goes up the git log until the test succeed.

## How it works

This salmon takes four arguments : git url, branch, commit, test script path. Salmon clones git repository with given branch, and checkout the given commit. Then it makes test with given test script path. When the test fails, salmon goes up to its parent commit and test again. It will be repeated until test succeed or there is no test script with given path

## How to build & test
``` bash
cargo run <github url> <branch name> <commit hash> <test script name = "test.sh">
```
After get binary, you can run it with 
``` bash
./salmon <github url> <branch name> <commit hash> <test script name = "test.sh">
```
test script name is based on the name of the test script in the git repository.


## Example
``` bash
cargo run https://github.com/roquen4145/Salmon-Test test d07f83fc3e83bcbc1750fd6782fa5faeaa75cdc8 test.sh
```