#!/bin/bash
set -ex

URL="https://github.com/onnx/models/raw/main/validated/vision/classification/mobilenet/model/mobilenetv2-7.onnx?download=:"
TGT="src/backend/assets/mobilenetv2-7.onnx"

echo "Downloading ${TGT}..."
if [ -s "${TGT}" ]; then
    echo "    (cached)"
    exit 0
fi

if which wget >/dev/null; then
    wget $URL -O $TGT
elif which curl >/dev/null; then
    curl -vL $URL -o $TGT
else
    echo "Couldn't find wget or curl."
    echo "Please download manually from \"$URL\" and save the file in $TGT."
fi
