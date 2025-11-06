#!/bin/bash

set -e

SCHEMA_FILE="database/schema/db.sql"
MIGRATIONS_DIR="database/migrations"
TMP_DB="atlas_tmp_$(date +%s)"
DEV_DB="atlas_dev_$(date +%s)"
DB_URL="postgresql://postgres:pass@localhost/$TMP_DB?sslmode=disable&search_path=public"
DEV_URL="postgresql://postgres:pass@localhost/$DEV_DB?sslmode=disable&search_path=public"
TIMESTAMP=$(date +%Y%m%d%H%M%S)

# Check prerequisites
if [[ ! -f "$SCHEMA_FILE" ]]; then
  echo "Error: $SCHEMA_FILE not found"
  exit 1
fi

mkdir -p "$MIGRATIONS_DIR"

# Count existing migrations
MIGRATION_COUNT=$(ls -1 "$MIGRATIONS_DIR"/*.sql 2>/dev/null | wc -l)

if [[ $MIGRATION_COUNT -eq 0 ]]; then
  # First migration - just copy db.sql
  echo "Creating initial migration..."
  cp "$SCHEMA_FILE" "$MIGRATIONS_DIR/${TIMESTAMP}_initial.sql"
  echo "✓ Created ${TIMESTAMP}_initial.sql"
else
  # Generate migration from diff
  echo "Creating temporary database..."
  createdb -U postgres "$TMP_DB"
  createdb -U postgres "$DEV_DB"

  # Apply existing migrations
  echo "Applying existing migrations..."
  for migration in "$MIGRATIONS_DIR"/*.sql; do
    if ! psql -U postgres "$TMP_DB" <"$migration"; then
      echo "Error: Migration failed. Cleaning up..."
      dropdb -U postgres "$TMP_DB"
      dropdb -U postgres "$DEV_DB"
      exit 1
    fi
  done

  # Generate new migration
  NEW_MIGRATION="$MIGRATIONS_DIR/${TIMESTAMP}_update.sql"

  echo "Generating migration diff..."
  if ! atlas schema diff \
    --from "$DB_URL" \
    --to "file://$SCHEMA_FILE" \
    --dev-url "$DEV_URL" \
    >"$NEW_MIGRATION"; then
    dropdb -U postgres "$TMP_DB"
    dropdb -U postgres "$DEV_DB"
    rm -f "$NEW_MIGRATION"
    exit 1
  fi

  # Check if schemas are in sync (no changes)
  if grep -q "Schemas are synced" "$NEW_MIGRATION" || [ ! -s "$NEW_MIGRATION" ]; then
    echo "✓ No changes detected - schemas are in sync"
    rm -f "$NEW_MIGRATION"
  else
    echo "✓ Created ${NEXT_NUM}_update.sql"
  fi

  # Cleanup
  dropdb -U postgres "$TMP_DB"
  dropdb -U postgres "$DEV_DB"

fi

echo "Done!"
