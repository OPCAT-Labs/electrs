#!/usr/bin/env bash
set -euo pipefail

# Fetch cookie from GCP Secret Manager if configured
COOKIE_ARG=""
if [ -n "${ELECTRS_COOKIE_SECRET:-}" ]; then
  COOKIE=$(gcloud secrets versions access latest --secret="${ELECTRS_COOKIE_SECRET}" 2>/dev/null) || {
    echo "Failed to fetch secret ${ELECTRS_COOKIE_SECRET}" >&2
    exit 1
  }
  COOKIE_ARG="--cookie=${COOKIE}"
fi

exec /usr/local/bin/electrs ${ELECTRS_ARGS:-} ${COOKIE_ARG}
