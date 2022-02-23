.PHONY: test
test:
	echo "BUILD_ENV is ${BUILD_ENV}"
	sh ./pre_deploy.sh
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
	dfx canister call encrypted_notes_motoko whoami
	sh test_whoami.sh
	echo "ENCRYPTED NOTES TEST PASSED"