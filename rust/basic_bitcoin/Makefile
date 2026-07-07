IMAGE_NAME = icp-cli-network-launcher-bitcoin

.PHONY: build-image topup

build-image:
	# Because we're building off of :latest, use --pull to fetch the latest image.
	# Remove `--pull` if the Dockerfile is updated to pin the base image version.
	docker build --pull -t $(IMAGE_NAME) .

topup:
	icp canister top-up --amount 30t backend
