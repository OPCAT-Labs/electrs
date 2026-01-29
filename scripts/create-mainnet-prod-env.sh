#!/usr/bin/env bash
set -euo pipefail

ENV_NAME="mainnet-prod"
ENV_FILE=""

usage() {
  cat <<'EOF'
Usage:
  scripts/create-mainnet-prod-env.sh [--env-file path]

Required variables (set as env vars or via --env-file):
  GCP_PROJECT_ID
  GCE_INSTANCE_LIST (comma-separated; supports name@zone)
  GCP_WIF_PROVIDER
  GCP_WIF_SERVICE_ACCOUNT

Optional (required only if any instance lacks @zone):
  GCE_ZONE

Optional variables:
  ELECTRS_SERVICE
  ELECTRS_ARGS
  ELECTRS_USER
  ELECTRS_WORKDIR
  ELECTRS_DATA_DIR (database directory, e.g., /mnt/electrs-data)
  LAYER_RPC_USER (RPC username)
  LAYER_RPC_PASSWORD_SECRET (GCP Secret Manager secret name for RPC password)
  ELECTRS_AFTER (systemd service dependency, e.g., layerd.service)

Notes:
  - Requires GitHub CLI (gh) logged in with repo access.
  - Creates the GitHub environment mainnet-prod if missing.
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

while [[ $# -gt 0 ]]; do
  case "$1" in
    --env-file)
      ENV_FILE="$2"
      shift 2
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage
      exit 1
      ;;
  esac
done

if ! command -v gh >/dev/null 2>&1; then
  echo "gh CLI is required" >&2
  exit 1
fi

if [[ -n "${ENV_FILE}" ]]; then
  if [[ ! -f "${ENV_FILE}" ]]; then
    echo "Env file not found: ${ENV_FILE}" >&2
    exit 1
  fi
  set -a
  # shellcheck disable=SC1090
  source "${ENV_FILE}"
  set +a
fi

REPO="${GH_REPO:-}"
if [[ -z "${REPO}" ]]; then
  REPO=$(gh repo view --json nameWithOwner -q .nameWithOwner)
fi

if [[ -z "${REPO}" ]]; then
  echo "Unable to determine repo; set GH_REPO=owner/name" >&2
  exit 1
fi

missing=()
for key in GCP_PROJECT_ID GCE_INSTANCE_LIST GCP_WIF_PROVIDER GCP_WIF_SERVICE_ACCOUNT; do
  if [[ -z "${!key:-}" ]]; then
    missing+=("${key}")
  fi
done

if [[ -z "${GCE_ZONE:-}" ]]; then
  has_zoneless=0
  IFS=',' read -r -a INSTANCES <<< "${GCE_INSTANCE_LIST:-}"
  for INSTANCE in "${INSTANCES[@]}"; do
    if [[ "${INSTANCE}" != *"@"* ]]; then
      has_zoneless=1
      break
    fi
  done
  if [[ ${has_zoneless} -eq 1 ]]; then
    missing+=("GCE_ZONE")
  fi
fi

if [[ ${#missing[@]} -gt 0 ]]; then
  echo "Missing required variables: ${missing[*]}" >&2
  exit 1
fi

# Create or update environment
printf "Creating environment %s in %s...\n" "${ENV_NAME}" "${REPO}"
gh api -X PUT "repos/${REPO}/environments/${ENV_NAME}" >/dev/null

set_variable() {
  local name="$1"
  local value="$2"
  printf "Setting variable %s...\n" "${name}"
  gh variable set --env "${ENV_NAME}" "${name}" --body "${value}" >/dev/null
}

# Variables (non-sensitive configuration)
set_variable "GCP_PROJECT_ID" "${GCP_PROJECT_ID}"
if [[ -n "${GCE_ZONE:-}" ]]; then
  set_variable "GCE_ZONE" "${GCE_ZONE}"
fi
set_variable "GCE_INSTANCE_LIST" "${GCE_INSTANCE_LIST}"
set_variable "GCP_WIF_PROVIDER" "${GCP_WIF_PROVIDER}"
set_variable "GCP_WIF_SERVICE_ACCOUNT" "${GCP_WIF_SERVICE_ACCOUNT}"

if [[ -n "${ELECTRS_SERVICE:-}" ]]; then
  set_variable "ELECTRS_SERVICE" "${ELECTRS_SERVICE}"
fi

if [[ -n "${ELECTRS_USER:-}" ]]; then
  set_variable "ELECTRS_USER" "${ELECTRS_USER}"
fi

if [[ -n "${ELECTRS_WORKDIR:-}" ]]; then
  set_variable "ELECTRS_WORKDIR" "${ELECTRS_WORKDIR}"
fi

if [[ -n "${ELECTRS_DATA_DIR:-}" ]]; then
  set_variable "ELECTRS_DATA_DIR" "${ELECTRS_DATA_DIR}"
fi

if [[ -n "${LAYER_RPC_USER:-}" ]]; then
  set_variable "LAYER_RPC_USER" "${LAYER_RPC_USER}"
fi

if [[ -n "${LAYER_RPC_PASSWORD_SECRET:-}" ]]; then
  set_variable "LAYER_RPC_PASSWORD_SECRET" "${LAYER_RPC_PASSWORD_SECRET}"
fi

if [[ -n "${ELECTRS_ARGS:-}" ]]; then
  set_variable "ELECTRS_ARGS" "${ELECTRS_ARGS}"
fi

if [[ -n "${ELECTRS_AFTER:-}" ]]; then
  set_variable "ELECTRS_AFTER" "${ELECTRS_AFTER}"
fi

printf "Done. Environment %s configured.\n" "${ENV_NAME}"
