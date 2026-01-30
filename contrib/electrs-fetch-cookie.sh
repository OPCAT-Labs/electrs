#!/usr/bin/env bash
set -euo pipefail

# Build ELECTRS_ARGS with cookie from GCP Secret Manager
# This script is called by ExecStartPre in systemd
# RuntimeDirectory=electrs ensures /run/electrs exists

ENV_FILE="/run/electrs/env"

# Create directory (runs as root via ExecStartPre=+)
mkdir -p "$(dirname "${ENV_FILE}")"

# Start with base args
ARGS="${ELECTRS_ARGS:-}"

# Append cookie from Secret Manager if configured
if [ -n "${LAYER_RPC_USER:-}" ] && [ -n "${LAYER_RPC_PASSWORD_SECRET:-}" ]; then
  PROJECT_ARG=""
  if [ -n "${GCP_PROJECT_ID:-}" ]; then
    PROJECT_ARG="--project=${GCP_PROJECT_ID}"
  fi
  # Run gcloud as service user to access their credentials
  SERVICE_USER="${SERVICE_USER:-opcat}"
  RPC_PASSWORD=$(sudo -u "${SERVICE_USER}" gcloud secrets versions access latest --secret="${LAYER_RPC_PASSWORD_SECRET}" ${PROJECT_ARG} 2>/dev/null) || {
    echo "Failed to fetch secret ${LAYER_RPC_PASSWORD_SECRET}" >&2
    exit 1
  }
  ARGS="${ARGS} --cookie=${LAYER_RPC_USER}:${RPC_PASSWORD}"
fi

echo "ELECTRS_STARTUP_ARGS=${ARGS}" > "${ENV_FILE}"
chmod 600 "${ENV_FILE}"
chown "${SERVICE_USER:-opcat}:${SERVICE_USER:-opcat}" "${ENV_FILE}"
