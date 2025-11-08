FROM alpine:latest

# Install Atlas
RUN apk add --no-cache bash curl
RUN curl -sSf https://atlasgo.sh | sh

# Copy migrations
COPY migrations /migrations
COPY entrypoint.sh /scripts/entrypoint.sh
RUN chmod +x /scripts/entrypoint.sh

WORKDIR /
ENTRYPOINT ["/scripts/entrypoint.sh"]
