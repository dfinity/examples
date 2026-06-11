FROM ghcr.io/dfinity/icp-cli-network-launcher:14.0.0-2026-06-04-04-52

RUN apt-get update && apt-get install -y bitcoind && rm -rf /var/lib/apt/lists/*

COPY docker/start.sh /app/start.sh
RUN chmod +x /app/start.sh

ENTRYPOINT ["/app/start.sh"]
