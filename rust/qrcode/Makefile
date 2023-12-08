.PHONY: all
all: deploy

.PHONY: node_modules
.SILENT: node_modules
node_modules:
	npm install

.PHONY: deploy
.SILENT: deploy
deploy: node_modules
	dfx deploy

.PHONY: test
.SILENT: test
test: deploy
	# Wait at least 2 seconds.
	sleep 2
	# Validate the image is generated as a query.
	dfx canister call qrcode_backend qrcode_query '("test", record {add_gradient=false; add_logo=false})' | fgrep -q 'Image = blob' && echo PASS
	dfx canister call qrcode_backend qrcode_query '("test", record {add_gradient=false; add_logo=true})' | fgrep -q 'Image = blob' && echo PASS
	dfx canister call qrcode_backend qrcode_query '("test", record {add_gradient=true; add_logo=false})' | fgrep -q 'Image = blob' && echo PASS
	dfx canister call qrcode_backend qrcode_query '("test", record {add_gradient=true; add_logo=true})' | fgrep -q 'Image = blob' && echo PASS
	dfx canister call qrcode_backend qrcode_query '("test", record {add_gradient=true; add_logo=true; add_trasparency=opt true})' | fgrep -q 'Image = blob' && echo PASS
	dfx canister call qrcode_backend qrcode_query '("test", record {add_gradient=true; add_logo=true; add_trasparency=opt false})' | fgrep -q 'Image = blob' && echo PASS
	# Validate the image is generated as an update call.
	dfx canister call qrcode_backend qrcode '("test", record {add_gradient=true; add_logo=true})' | fgrep -q 'Image = blob' && echo PASS

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
