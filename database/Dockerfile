FROM arigaio/atlas:latest

COPY migrations /migrations
COPY scripts/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

WORKDIR /
ENTRYPOINT ["/entrypoint.sh"]

