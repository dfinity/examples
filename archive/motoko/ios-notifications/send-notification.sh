#!/bin/zsh

source "./apple.env"

TOKEN_KEY_FILE_NAME="certificate.pem" # the certificate can be downloaded when creating a new key on apple developer website
TEAM_ID="${TEAM_ID}" # your account team id (available under https://developer.apple.com/account/#membership-details)
AUTH_KEY_ID="${AUTH_KEY_ID}" # your certificate key id (available under https://developer.apple.com/account/resources/authkeys) 
TOPIC="${TOPIC}" # your app bundle id
DEVICE_TOKEN="${DEVICE_TOKEN}" # the device id is printed to the xcode console for this example
APNS_HOST_NAME="api.sandbox.push.apple.com"

JWT_ISSUE_TIME=$(date +%s)
JWT_HEADER=$(printf '{ "alg": "ES256", "kid": "%s" }' "${AUTH_KEY_ID}" | openssl base64 -e -A | tr -- '+/' '-_' | tr -d =)
JWT_CLAIMS=$(printf '{ "iss": "%s", "iat": %d }' "${TEAM_ID}" "${JWT_ISSUE_TIME}" | openssl base64 -e -A | tr -- '+/' '-_' | tr -d =)
JWT_HEADER_CLAIMS="${JWT_HEADER}.${JWT_CLAIMS}"
JWT_SIGNED_HEADER_CLAIMS=$(printf "${JWT_HEADER_CLAIMS}" | openssl dgst -binary -sha256 -sign "${TOKEN_KEY_FILE_NAME}" | openssl base64 -e -A | tr -- '+/' '-_' | tr -d =)
AUTHENTICATION_TOKEN="${JWT_HEADER}.${JWT_CLAIMS}.${JWT_SIGNED_HEADER_CLAIMS}"

# Notification details
NOTIFICATION_TITLE="Update"
NOTIFICATION_BODY="Example dapp navigation to about"
NOTIFICATION_URL="https://ptf55-faaaa-aaaag-qbd6q-cai.ic0.app?route=about"

curl -v --header "apns-topic: $TOPIC" --header "apns-push-type: alert" --header "apns-collapse-id: test-message" --header "authorization: bearer $AUTHENTICATION_TOKEN" --data "{\"aps\":{\"alert\":{\"title\":\"$NOTIFICATION_TITLE\",\"body\":\"$NOTIFICATION_BODY\",\"url\":\"$NOTIFICATION_URL\"}}}" --http2 https://${APNS_HOST_NAME}/3/device/${DEVICE_TOKEN}
