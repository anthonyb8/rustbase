FROM arigaio/atlas:latest

COPY migrations /migrations

WORKDIR /

# This ensures variable expansion happens at runtime
ENTRYPOINT ["/bin/sh", "-c"]
CMD ["migrate apply --url \"$POSTGRES_URL\" --dir file:///migrations"]
