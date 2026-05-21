#!/bin/bash

# In GitHub Codespaces, inject the forwarded domain into icp.yaml so the
# HTTP gateway accepts requests with that Host header.
if [ -n "$CODESPACE_NAME" ]; then
  DOMAIN="${CODESPACE_NAME}-8000.app.github.dev"
  node -e "
    const fs = require('fs');
    const content = fs.readFileSync('icp.yaml', 'utf8');
    if (!content.includes('gateway:')) {
      const gateway = '    gateway:\n      domains:\n        - localhost\n        - ${DOMAIN}\n';
      let updated;
      if (content.includes('    ii: true\n')) {
        updated = content.replace('    ii: true\n', '    ii: true\n' + gateway);
      } else {
        updated = content.replace('    mode: managed\n', '    mode: managed\n' + gateway);
      }
      fs.writeFileSync('icp.yaml', updated);
    }
  "
fi

icp network start -d
