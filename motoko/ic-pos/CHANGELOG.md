# Changelog

## [0.3.0] - 2022-09-10

### Added

- Now you can charge a set amount, similar to Sumup and the likes. Enter the amount to charge and click the "Charge" button. The generated QR code will contain information about the token and amount to charge. The customer can then scan and pay with any wallet that supports the QR format, NNS for instance. Raad more about ICRC-22 payment requests: https://github.com/dfinity/ICRC/issues/22

### Fixed

- Improved printing styles, QR code now fills more of the page.

## [0.2.0] - 2022-09-06

### Added

- Principal pills are now clickable. Click to copy the principal id to the clipboard.

### Changed

- The `icpos` canister continuoulsy monitors the ckBTC ledger and sends notifications to merchants that choosen to receive email or sms. The ledger id used to be hardcode, now there is a function `setLedgerId` that can be called at runtime.
- `dfx.json`: settings now included to run an ICRC-1 ledger and index locally.
- Upgraded to latest versions of `@dfinity/xxx` packages.
- Removed reliance on the HTTP ckBTC transaction API. Instead, IC-POS now uses the ledger canister directly to monitor transactions.
- ckBTC balances are now updated in the UI after received transfers.

## [0.1.0]

Initial release.
