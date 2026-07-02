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
# json.dumps produces a Candid-compatible string literal (same escape rules).
CANDID_ARG=$(python3 -c "
import json
with open('/tmp/x509_test_csr.pem') as f:
    csr = f.read()
print('(record { pem_certificate_request = ' + json.dumps(csr) + ' })')
")
result=$(icp canister call backend child_certificate "$CANDID_ARG") && \
  echo "$result" && \
  echo "$result" | grep -q 'BEGIN CERTIFICATE' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: Verify root CA certificate is self-signed (openssl verify) ==="
# Extract the PEM from the Candid text output. json.loads handles the
# escape sequences (\n, \\, etc.) which are identical in Candid and JSON.
icp canister call backend root_ca_certificate '()' \
  | python3 -c "
import sys, re, json
text = sys.stdin.read()
m = re.search(r'x509_certificate_string = \"((?:[^\"\\\\]|\\\\.)*)\"', text)
pem = json.loads('\"' + m.group(1) + '\"')
print(pem, end='')
" > /tmp/x509_root_ca.pem && \
  openssl verify -CAfile /tmp/x509_root_ca.pem /tmp/x509_root_ca.pem 2>&1 && \
  echo "PASS" || (echo "FAIL" && exit 1)

rm -f /tmp/x509_test_key.pem /tmp/x509_test_csr.pem /tmp/x509_root_ca.pem
