# Always use the latest network launcher image
# Before building we must pull to pick up new releases
# For real world usage, consider pinning the version instead of using :latest
FROM ghcr.io/dfinity/icp-cli-network-launcher:latest

ARG TARGETARCH
ARG BITCOIN_VERSION=27.2

RUN apt-get update && apt-get install -y --no-install-recommends curl && \
    case "${TARGETARCH}" in \
      "amd64") \
        BITCOIN_TARBALL="bitcoin-${BITCOIN_VERSION}-x86_64-linux-gnu.tar.gz" ; \
        BITCOIN_SHA256="acc223af46c178064c132b235392476f66d486453ddbd6bca6f1f8411547da78" ;; \
      "arm64") \
        BITCOIN_TARBALL="bitcoin-${BITCOIN_VERSION}-aarch64-linux-gnu.tar.gz" ; \
        BITCOIN_SHA256="154c9b9e6e17136edc8f20fda5d252fb339e727e4a85ef49e7d8facb9085f2d3" ;; \
      *) echo "Unsupported architecture: ${TARGETARCH}" && exit 1 ;; \
    esac && \
    curl -fsSL "https://bitcoincore.org/bin/bitcoin-core-${BITCOIN_VERSION}/${BITCOIN_TARBALL}" \
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
