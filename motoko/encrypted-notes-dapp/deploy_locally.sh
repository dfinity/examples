#!/bin/bash

# turn on bash's job control

BUILD_ENV=${BUILD_ENV:-motoko}
export BUILD_ENV

source pre_deploy.sh

echo "--- docker build ---"
docker build --build-arg BUILD_ENV=${BUILD_ENV} -t encrypted_notes .

echo "--- docker run ---"
DOCKER_IMAGE=$(docker run -v $(pwd):/canister -it -d -p 8080:8080 -p 8000:8000 -p 3000:3000 -p 35729:35729 -e BUILD_ENV=${BUILD_ENV} --rm encrypted_notes)
echo "Created Docker instance: ${DOCKER_IMAGE} (exported)"
export DOCKER_IMAGE

echo "--- deploy ---"
docker exec "${DOCKER_IMAGE}" sh deploy_locally_impl.sh
