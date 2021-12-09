# Salmon

Like salmon leaping up the fall, this program goes up the git log until the test succeed.

## How it works

This salmon takes four arguments : git url, branch, commit, test script path. Salmon clones git repository with given branch, and checkout the given commit. Then it makes test with given test script path. When the test fails, salmon goes up to its parent commit and test again. It will be repeated until test succeed or there is no test script with given path