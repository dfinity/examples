#!/usr/bin/env bash

set -e

# dfx canister call invoice accountIdentifierToBlob '(variant {"text" = "F482E833ADB604E72AF08504AAA46C0BB6D51706CA9C4BB83F79A8F7393171B2"})'


dfx canister call ledger transfer '( record { memo = 0; amount = record { e8s = 10_000_000_000 }; fee = record { e8s = 10000 }; to = blob "\f84\bd0t\22\e4r%\d9\15\88\80\92\81\0b>\ae\9d\ae\a5\e5Kg\df\c9\97\99i\8f\8e\ea" } )'
