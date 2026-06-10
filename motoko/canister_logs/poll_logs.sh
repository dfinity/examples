#!/bin/bash

# Continuously polls and displays new canister log entries.
# Useful for watching logs stream in real-time while calling canister methods
# in a separate terminal.

declare -a previous_logs=()

fetch_and_filter_logs() {
    local new_logs
    new_logs=$(icp canister logs backend)

    while IFS= read -r line; do
        if [[ ! "${previous_logs[*]}" =~ "$line" ]]; then
            echo "$line"
        fi
    done <<< "$new_logs"

    previous_logs=("$new_logs")
}

fetch_and_filter_logs

while true; do
    fetch_and_filter_logs
    sleep 1
done
