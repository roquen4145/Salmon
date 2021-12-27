#!/bin/bash
# Test script for Salmon-Test repository
# You can edit this file to test your repostitory 
# or create new test files

# Test : get content of file
result=`cat temp`

# Wanted content is 'b'
if [ "$result" = "b" ]; then
    exit 0 # exit with 0 when test is ok
else
    exit 1 # failure, salmon will checkout to past commit
fi