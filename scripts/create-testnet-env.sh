#!/usr/bin/env bash
set -euo pipefail

ENV_NAME="testnet"
ENV_FILE=""

usage() {
  cat <<'EOF'
Usage:
  scripts/create-testnet-env.sh [--env-file path]

Required (set as env vars or via --env-file, see scripts/testnet.env.example):
  DO_HOST_LIST       comma-separated droplet public addresses (stored as secret)
  ELECTRS_ARGS       (stored as secret; contains VPC addresses)
  LAYER_RPC_USER     (stored as secret)
  LAYER_RPC_PASSWORD (stored as secret)
  DO_SSH_KEY_FILE    path to the per-droplet CD private key (stored as secret)

Optional:
  ELECTRS_USER       (default: opcat)
  DO_KNOWN_HOSTS_FILE  pinned host keys; if unset, ssh-keyscan is run against DO_HOST_LIST

Notes:
  - Requires GitHub CLI (gh) logged in with repo access.
  - Creates the GitHub environment testnet if missing.
  - Host and VPC addresses are stored as secrets so they are masked in public workflow logs.
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
for key in DO_HOST_LIST ELECTRS_ARGS LAYER_RPC_USER LAYER_RPC_PASSWORD DO_SSH_KEY_FILE; do
  if [[ -z "${!key:-}" ]]; then
    missing+=("${key}")
  fi
done

if [[ ${#missing[@]} -gt 0 ]]; then
  echo "Missing required variables: ${missing[*]}" >&2
  exit 1
fi

if [[ ! -f "${DO_SSH_KEY_FILE}" ]]; then
  echo "SSH key file not found: ${DO_SSH_KEY_FILE}" >&2
  exit 1
fi

if [[ -n "${DO_KNOWN_HOSTS_FILE:-}" ]]; then
  KNOWN_HOSTS=$(cat "${DO_KNOWN_HOSTS_FILE}")
else
  KNOWN_HOSTS=$(ssh-keyscan -t ed25519 $(echo "${DO_HOST_LIST}" | tr ',' ' ') 2>/dev/null)
fi

if [[ -z "${KNOWN_HOSTS}" ]]; then
  echo "Could not collect host keys; set DO_KNOWN_HOSTS_FILE" >&2
  exit 1
fi

# Create or update environment
printf "Creating environment %s in %s...\n" "${ENV_NAME}" "${REPO}"
gh api -X PUT "repos/${REPO}/environments/${ENV_NAME}" >/dev/null

set_variable() {
  local name="$1"
  local value="$2"
  printf "Setting variable %s...\n" "${name}"
  gh variable set --repo "${REPO}" --env "${ENV_NAME}" "${name}" --body "${value}" >/dev/null
}

set_secret() {
  local name="$1"
  local value="$2"
  printf "Setting secret %s...\n" "${name}"
  printf '%s' "${value}" | gh secret set --repo "${REPO}" --env "${ENV_NAME}" "${name}" >/dev/null
}

# Variables (non-sensitive configuration)
set_variable "ELECTRS_USER" "${ELECTRS_USER:-opcat}"

# Secrets
set_secret "ELECTRS_ARGS" "${ELECTRS_ARGS}"
set_secret "LAYER_RPC_USER" "${LAYER_RPC_USER}"
set_secret "DO_HOST_LIST" "${DO_HOST_LIST}"
set_secret "DO_KNOWN_HOSTS" "${KNOWN_HOSTS}"
set_secret "LAYER_RPC_PASSWORD" "${LAYER_RPC_PASSWORD}"
set_secret "DO_SSH_KEY" "$(cat "${DO_SSH_KEY_FILE}")"

printf "Done. Environment %s configured.\n" "${ENV_NAME}"
