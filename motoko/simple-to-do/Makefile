.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create simple_to_do
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install simple_to_do

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install simple_to_do --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	# Add To-Dos.
	dfx canister call simple_to_do addTodo '("Create a project")'
	dfx canister call simple_to_do addTodo '("Build the project")'
	dfx canister call simple_to_do addTodo '("Deploy the project")'
	# Show To-Dos.
	dfx canister call simple_to_do showTodos \
		| grep '(1) Create a project' && echo 'PASS'
	dfx canister call simple_to_do showTodos \
		| grep '(2) Build the project' && echo 'PASS'
	dfx canister call simple_to_do showTodos \
		| grep '(3) Deploy the project' && echo 'PASS'
	# Complete To-Dos.
	dfx canister call simple_to_do completeTodo '(1)'
	dfx canister call simple_to_do completeTodo '(2)'
	dfx canister call simple_to_do completeTodo '(3)'
	# Show To-Dos.
	dfx canister call simple_to_do showTodos \
		| grep '(1) Create a project ✔' && echo 'PASS'
	dfx canister call simple_to_do showTodos \
		| grep '(2) Build the project ✔' && echo 'PASS'
	dfx canister call simple_to_do showTodos \
		| grep '(3) Deploy the project ✔' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
