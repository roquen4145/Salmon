#!/bin/bash
# Test script for Salmon-Test repository

# Test : get content of file
result=`cat temp`

# Wanted content is 'a'
if [ "$result" = "a" ]; then
    exit 0 # success
else
    exit 1 # failure, salmon will checkout to past commit
fi