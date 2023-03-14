.PHONY: check docs test e2e 

check:
	find src -type f -name '*.mo' -print0 | xargs -0 $(shell vessel bin)/moc $(shell vessel sources) --check

all: check-strict docs test e2e

check-strict:
	find src -type f -name '*.mo' -print0 | xargs -0 $(shell vessel bin)/moc $(shell vessel sources) -Werror --check

docs:
	$(shell vessel bin)/mo-doc --output docs/generated

test:
	$(shell vessel bin)/moc -r $(shell vessel sources) -wasi-system-api test/unit/Test.mo

#npm run deployForTesting also runs dfx generate invoice
#npm run test triggers `cd test/e2e; npm ci; npm test` which in turn triggers `vitest run`
e2e:
	npm ci
	npm run deployForTesting
	npm run test 

watch:
	while true; do \
		make $(WATCHMAKE); \
		inotifywait --exclude **/.vessel -qre close_write .; \
	done