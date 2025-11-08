#!/bin/bash

ENV_FILE=".env"

aws ssm get-parameters-by-path --path "/testing/prod" --with-decryption \
  --query "Parameters[*].[Name,Value]" --output text | while read name value; do
  key=$(echo "$name" | awk -F'/' '{print toupper($NF)}')
  echo "$key=$value"
done >"$ENV_FILE"

if ! docker compose up --build -d; then
  rm -rf "$ENV_FILE"
  exit 1
fi

rm -rf "$ENV_FILE"

# docker build -t backend ./backend
# docker tag backend:latest 186091750176.dkr.ecr.us-east-1.amazonaws.com/backend:latest
# docker push 186091750176.dkr.ecr.us-east-1.amazonaws.com/backend:latest
