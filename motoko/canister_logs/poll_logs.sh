#!/bin/bash

# Continuously polls and displays new canister log entries.
# Useful for watching logs stream in real-time while calling canister methods
# in a separate terminal.
#
# Requires: jq (https://jqlang.org)

if ! command -v jq &>/dev/null; then
  echo "Error: jq is required. Install it with: brew install jq  (macOS) or apt install jq (Linux)"
  exit 1
fi

last_index=-1

while true; do
  logs=$(icp canister logs backend 2>/dev/null)
  if [ -n "$logs" ]; then
    # Print only entries with index > last seen
    new_entries=$(echo "$logs" | jq -r --argjson last "$last_index" \
      '.log_records[] | select(.index > $last) | "[\(.index)]: \(.content)"')
    if [ -n "$new_entries" ]; then
      echo "$new_entries"
      last_index=$(echo "$logs" | jq '.log_records[-1].index // -1')
    fi
  fi
  sleep 1
done
