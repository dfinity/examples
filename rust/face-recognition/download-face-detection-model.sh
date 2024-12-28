#!/bin/bash
set -e

ULTRAFACE_URL="https://github.com/onnx/models/raw/bec48b6a70e5e9042c0badbaafefe4454e072d08/validated/vision/body_analysis/ultraface/models/version-RFB-320.onnx"
ULTRAFACE_TGT="version-RFB-320.onnx"

echo "Downloading ${ULTRAFACE_TGT}..."
if [ -s "${ULTRAFACE_TGT}" ]; then
    echo "    (cached)"
    exit 0
fi

if which wget >/dev/null; then
    wget $ULTRAFACE_URL -O $ULTRAFACE_TGT
elif which curl >/dev/null; then
    curl -vL $ULTRAFACE_URL -o $ULTRAFACE_TGT
else
    echo "Couldn't find wget or curl."
    echo "Please download manually from \"$ULTRAFACE_URL\" and save the file in $ULTRAFACE_TGT."
fi
