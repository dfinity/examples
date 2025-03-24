BUILD_ENV := motoko

.PHONY: test test-unit test-e2e

test: test-unit test-e2e

test-unit:
	echo "BUILD_ENV is ${BUILD_ENV}"
	bash ./src/encrypted_notes_${BUILD_ENV}/test/run_tests.sh
	echo "ENCRYPTED NOTES UNIT TESTS PASSED"

test-e2e:
	echo "BUILD_ENV is ${BUILD_ENV}"
	bash ./pre_deploy.sh
	echo "Pre deploy script succeeded"
	npm install
	rm -fr .dfx
	dfx start --clean --background
	dfx deploy internet_identity --argument '(null)'
	dfx deploy encrypted_notes_${BUILD_ENV}
	dfx generate encrypted_notes_${BUILD_ENV}
	dfx deploy www
	echo "Deployment succeeded"
	echo "Start testing..."
	dfx canister call encrypted_notes_${BUILD_ENV} whoami
	sh test_whoami.sh
	echo "ENCRYPTED NOTES E2E TESTS PASSED"
