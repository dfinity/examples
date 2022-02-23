.PHONY: test
test:
	echo "BUILD_ENV is ${BUILD_ENV}"
	sh ./pre_deploy.sh
	echo "pre deploy succeeded"
	sh ./deploy_locally_impl.sh
	echo "deploy locally succeeded"
	echo "ENCRYPTED NOTES TEST PASSED"