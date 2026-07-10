#!/usr/bin/env bash
set -e
# Uses --pull to always fetch the latest base image.
# Remove --pull if the Dockerfile is updated to pin the base image version.
docker build --pull -t icp-cli-network-launcher-bitcoin .
