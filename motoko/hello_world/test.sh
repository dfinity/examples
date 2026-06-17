#!/usr/bin/env bash
set -e

echo "--- Testing default greeting ---"
result=$(icp canister call --query backend greet '("World")')
echo "$result"
echo "$result" | grep -q 'Hello, World!' && echo "PASS" || (echo "FAIL" && exit 1)

echo "--- Testing setGreeting ---"
icp canister call backend setGreeting '("Hi, ")'
result=$(icp canister call --query backend greet '("Alice")')
echo "$result"
echo "$result" | grep -q 'Hi, Alice!' && echo "PASS" || (echo "FAIL" && exit 1)
