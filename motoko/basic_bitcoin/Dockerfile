FROM ghcr.io/dfinity/icp-cli-network-launcher:latest

ARG BITCOIN_VERSION=27.2
ARG BITCOIN_SHA256=acc223af46c178064c132b235392476f66d486453ddbd6bca6f1f8411547da78

RUN apt-get update && apt-get install -y --no-install-recommends curl && \
    curl -fsSL "https://bitcoincore.org/bin/bitcoin-core-${BITCOIN_VERSION}/bitcoin-${BITCOIN_VERSION}-x86_64-linux-gnu.tar.gz" \
      -o /tmp/bitcoin.tar.gz && \
    echo "${BITCOIN_SHA256}  /tmp/bitcoin.tar.gz" | sha256sum -c && \
    tar xzf /tmp/bitcoin.tar.gz --strip-components=2 \
      -C /usr/local/bin \
      "bitcoin-${BITCOIN_VERSION}/bin/bitcoind" \
      "bitcoin-${BITCOIN_VERSION}/bin/bitcoin-cli" && \
    rm /tmp/bitcoin.tar.gz && \
    apt-get purge -y curl && apt-get autoremove -y && rm -rf /var/lib/apt/lists/*

COPY docker/start.sh /app/start.sh
RUN chmod +x /app/start.sh

ENTRYPOINT ["/app/start.sh"]
