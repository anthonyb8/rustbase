#!/bin/bash

set -e

exec migrate apply --url "$POSTGRES_URL" --dir file:///migrations
