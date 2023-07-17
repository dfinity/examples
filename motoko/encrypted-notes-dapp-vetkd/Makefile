BUILD_ENV := motoko

.PHONY: test test-unit test-e2e

test: test-unit test-e2e

test-unit:
	echo "BUILD_ENV is ${BUILD_ENV}"
	bash ./src/encrypted_notes_${BUILD_ENV}/test/run_tests.sh
	echo "ENCRYPTED NOTES UNIT TESTS PASSED"

test-e2e:
	dfx stop
	echo "BUILD_ENV is ${BUILD_ENV}"
	bash ./pre_deploy.sh
	echo "Pre deploy script succeeded"
	npm install
	rm -fr .dfx
	dfx start --clean --background
	dfx deploy internet_identity --argument '(null)'
	dfx canister create vetkd_system_api --specified-id s55qq-oqaaa-aaaaa-aaakq-cai
	dfx deploy vetkd_system_api
	dfx deploy encrypted_notes_${BUILD_ENV}
	dfx generate encrypted_notes_${BUILD_ENV}
	dfx deploy www
	echo "Deployment succeeded"
	echo "Start testing..."
	dfx canister call encrypted_notes_${BUILD_ENV} whoami
	sh test_whoami.sh
	echo "ENCRYPTED NOTES E2E TESTS PASSED"

clean:
	rm -rf .dfx
	rm -rf node_modules
	rm -rf src/declarations
	rm -rf src/frontend/public/build
	rm -rf src/frontend/src/lib/backend.ts
	rm -rf src/frontend/src/lib/idlFactory.js
	rm -rf dfx.json
	cargo clean
