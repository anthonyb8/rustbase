FROM alpine:latest

# Install Atlas
RUN apk add --no-cache bash curl
RUN curl -sSf https://atlasgo.sh | sh

# Copy migrations
COPY migrations /migrations
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

WORKDIR /
ENTRYPOINT ["/entrypoint.sh"]
