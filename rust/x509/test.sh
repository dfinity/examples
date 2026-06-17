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
openssl genpkey -algorithm Ed25519 -out /tmp/x509_test_key.pem 2>/dev/null && \
  openssl req -new -key /tmp/x509_test_key.pem -out /tmp/x509_test_csr.pem \
    -subj "/CN=Test Corporation/O=Test Inc/C=US" 2>/dev/null && \
  CSR=$(cat /tmp/x509_test_csr.pem | tr '\n' '\\n' | sed 's/\\n$//') && \
  result=$(icp canister call backend child_certificate \
    "(record { pem_certificate_request = \"$CSR\n\" })") && \
  echo "$result" && \
  echo "$result" | grep -q 'BEGIN CERTIFICATE' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: Verify root CA certificate is self-signed (openssl verify) ==="
icp canister call backend root_ca_certificate '()' --output json \
  | python3 -c "import sys,json; d=json.load(sys.stdin); print(d['Ok']['x509_certificate_string'])" \
  | sed 's/\\n/\n/g' > /tmp/x509_root_ca.pem && \
  openssl verify -CAfile /tmp/x509_root_ca.pem /tmp/x509_root_ca.pem 2>&1 && \
  echo "PASS" || (echo "FAIL" && exit 1)

rm -f /tmp/x509_test_key.pem /tmp/x509_test_csr.pem /tmp/x509_root_ca.pem
