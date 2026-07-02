#!/usr/bin/env bash
set -e

echo "=== Test 1: Generate root CA certificate (Ed25519) ==="
result=$(icp canister call backend root_ca_certificate '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'BEGIN CERTIFICATE' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: Root CA certificate is idempotent (cached on second call) ==="
result=$(icp canister call backend root_ca_certificate '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'BEGIN CERTIFICATE' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: Generate and sign a child certificate from an Ed25519 CSR ==="
openssl genpkey -algorithm Ed25519 -out /tmp/x509_test_key.pem 2>/dev/null
openssl req -new -key /tmp/x509_test_key.pem -out /tmp/x509_test_csr.pem \
  -subj "/CN=Test Corporation/O=Test Inc/C=US" 2>/dev/null
# awk converts each newline to the two-character sequence \n so the PEM can
# be embedded as a Candid text string literal (Candid uses the same escapes as JSON).
CSR_ESCAPED=$(awk '{printf "%s\\n", $0}' /tmp/x509_test_csr.pem)
result=$(icp canister call backend child_certificate \
  "(record { pem_certificate_request = \"${CSR_ESCAPED}\" })") && \
  echo "$result" && \
  echo "$result" | grep -q 'BEGIN CERTIFICATE' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: Verify root CA certificate is self-signed (openssl verify) ==="
# Extract the PEM from the Candid output and convert \n back to real newlines.
icp canister call backend root_ca_certificate '()' \
  | grep 'x509_certificate_string' \
  | sed 's/.*x509_certificate_string = "//; s/"[;,]*$//; s/\\n/\n/g' \
  > /tmp/x509_root_ca.pem && \
  openssl verify -CAfile /tmp/x509_root_ca.pem /tmp/x509_root_ca.pem 2>&1 && \
  echo "PASS" || (echo "FAIL" && exit 1)

rm -f /tmp/x509_test_key.pem /tmp/x509_test_csr.pem /tmp/x509_root_ca.pem
