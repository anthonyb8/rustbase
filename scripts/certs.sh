#!/bin/bash

mkdir -p ./certs

# Generate CA and server certificates
mkcert -cert-file ./certs/server.crt -key-file ./certs/server.key postgres localhost 127.0.0.1
cp "$(mkcert -CAROOT)/rootCA.pem" ./certs/ca.crt

# Set proper permissions
chown 999:999 ./certs/server.key ./certs/server.crt ./certs/ca.crt
chmod 600 ./certs/server.key
chmod 644 ./certs/server.crt ./certs/ca.crt
