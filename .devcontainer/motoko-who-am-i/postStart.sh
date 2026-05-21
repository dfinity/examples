#!/bin/bash

# In GitHub Codespaces, inject the forwarded domain into icp.yaml so the
# HTTP gateway accepts requests with that Host header.
if [ -n "$CODESPACE_NAME" ]; then
  DOMAIN="${CODESPACE_NAME}-8000.app.github.dev"
  node -e "
    const fs = require('fs');
    const content = fs.readFileSync('icp.yaml', 'utf8');
    if (!content.includes('gateway:')) {
      const updated = content.replace(
        '    ii: true\n',
        '    ii: true\n    gateway:\n      domains:\n        - localhost\n        - ${DOMAIN}\n'
      );
      fs.writeFileSync('icp.yaml', updated);
    }
  "
fi

icp network start -d
