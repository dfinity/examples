#!/usr/bin/env bash
set -e

trap 'rm -f /tmp/x509_test_key.pem /tmp/x509_test_csr.pem /tmp/x509_root_ca.pem /tmp/x509_child_cert.pem' EXIT

echo "=== Test 1: Generate root CA certificate (Ed25519) ==="
root_cert=$(icp canister call backend root_ca_certificate '()') && \
  echo "$root_cert" && \
  echo "$root_cert" | grep -q 'BEGIN CERTIFICATE' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: Root CA certificate is idempotent (same on second call) ==="
root_cert2=$(icp canister call backend root_ca_certificate '()') && \
  echo "$root_cert2" && \
  [ "$root_cert" = "$root_cert2" ] && \
  echo "PASS" || (echo "FAIL: certificate changed between calls" && exit 1)

echo "=== Test 3: Generate and sign a child certificate from an Ed25519 CSR ==="
openssl genpkey -algorithm Ed25519 -out /tmp/x509_test_key.pem 2>/dev/null
openssl req -new -key /tmp/x509_test_key.pem -out /tmp/x509_test_csr.pem \
  -subj "/CN=Test Corporation/O=Test Inc/C=US" 2>/dev/null
# awk converts each newline to the two-character sequence \n so the PEM can
# be embedded as a Candid text string literal (same escaping rules as JSON).
CSR_ESCAPED=$(awk '{printf "%s\\n", $0}' /tmp/x509_test_csr.pem)
child_cert=$(icp canister call backend child_certificate \
  "(record { pem_certificate_request = \"${CSR_ESCAPED}\" })") && \
  echo "$child_cert" && \
  echo "$child_cert" | grep -q 'BEGIN CERTIFICATE' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: Verify cryptographic chain with OpenSSL ==="
# Extract the root CA PEM and verify it is self-signed.
echo "$root_cert" \
  | grep 'x509_certificate_string' \
  | sed 's/.*x509_certificate_string = "//; s/"[;,]*$//; s/\\n/\n/g' \
  > /tmp/x509_root_ca.pem
openssl verify -CAfile /tmp/x509_root_ca.pem /tmp/x509_root_ca.pem 2>&1

# Extract the child certificate PEM and verify it was signed by the root CA.
# This is the key check: the canister's threshold key signed a real X.509 cert.
echo "$child_cert" \
  | grep 'x509_certificate_string' \
  | sed 's/.*x509_certificate_string = "//; s/"[;,]*$//; s/\\n/\n/g' \
  > /tmp/x509_child_cert.pem
openssl verify -CAfile /tmp/x509_root_ca.pem /tmp/x509_child_cert.pem 2>&1 && \
  echo "PASS" || (echo "FAIL" && exit 1)

