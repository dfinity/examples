.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create --all
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install mat_mat_mul

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install mat_mat_mul --mode=upgrade

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx deploy mat_mat_mul

.PHONY: test
.SILENT: test
test: deploy
	# Validate the number of instructions is non-zero.
	dfx canister call mat_mat_mul naive_f32 | grep -vw '0' && echo 'PASS'
	dfx canister call mat_mat_mul optimized_f32 | grep -vw '0' && echo 'PASS'
	dfx canister call mat_mat_mul auto_vectorized_f32 | grep -vw '0' && echo 'PASS'
	dfx canister call mat_mat_mul simd_f32 | grep -vw '0' && echo 'PASS'
	dfx canister call mat_mat_mul naive_u32 | grep -vw '0' && echo 'PASS'
	dfx canister call mat_mat_mul optimized_u32 | grep -vw '0' && echo 'PASS'
	dfx canister call mat_mat_mul auto_vectorized_u32 | grep -vw '0' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
