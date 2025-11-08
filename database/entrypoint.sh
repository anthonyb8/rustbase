#!/bin/bash
set -e

if [ -z "$POSTGRES_URL" ]; then
  echo "ERROR: POSTGRES_URL is not set!" >&2
  exit 1
fi
echo "Using URL: ${POSTGRES_URL:0:20}..." >&2
exec atlas migrate apply --url "$POSTGRES_URL" --dir file:///migrations
