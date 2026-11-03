#!/usr/bin/env bash
set -euo pipefail

# Install an electrs release on a DigitalOcean droplet and (re)start the service.
#
# Installed once by root as /usr/local/sbin/electrs-deploy (root-owned), and the
# CD user is allowed to run exactly this script via sudoers:
#   opcat ALL=(root) NOPASSWD: /usr/local/sbin/electrs-deploy
#
# Usage: sudo electrs-deploy <tag> < env
#   <tag>  release tag, e.g. v1.2.3 or v1.2.3-rc1
#   stdin  optional KEY=VALUE lines for /etc/electrs/env; only ELECTRS_ARGS,
#          LAYER_RPC_USER and LAYER_RPC_PASSWORD are accepted. Empty stdin keeps
#          the existing env file.

REPO="OPCAT-Labs/electrs"
SERVICE_NAME="electrs"
SERVICE_USER="opcat"
ELECTRS_WORKDIR="/var/lib/electrs"
ENV_DIR="/etc/electrs"

TAG="${1:-}"
if [[ ! "${TAG}" =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.]+)?$ ]]; then
  echo "usage: electrs-deploy <tag like v1.2.3>" >&2
  exit 1
fi
VERSION="${TAG#v}"
NAME="electrs-v${VERSION}-linux-x86_64-musl"
BASE_URL="https://github.com/${REPO}/releases/download/${TAG}"

if ! id -u "${SERVICE_USER}" >/dev/null 2>&1; then
  echo "User ${SERVICE_USER} does not exist" >&2
  exit 1
fi

# Read env lines from stdin before doing anything else
NEW_ENV=""
if [ ! -t 0 ]; then
  while IFS= read -r line || [ -n "${line}" ]; do
    [ -z "${line}" ] && continue
    case "${line}" in
      ELECTRS_ARGS=*|LAYER_RPC_USER=*|LAYER_RPC_PASSWORD=*) NEW_ENV+="${line}"$'\n' ;;
      *) echo "Rejected env line: ${line%%=*}" >&2; exit 1 ;;
    esac
  done
fi

tmpdir=$(mktemp -d)
trap 'rm -rf "${tmpdir}"' EXIT
cd "${tmpdir}"

# Download and verify the release
curl -fsSL -o "${NAME}.tar.gz" "${BASE_URL}/${NAME}.tar.gz"
curl -fsSL -o SHA256SUMS "${BASE_URL}/SHA256SUMS"
sha256sum -c SHA256SUMS --ignore-missing
tar xzf "${NAME}.tar.gz"

install -m 0755 "${NAME}/bin/electrs" /usr/local/bin/electrs
if [ -f "${NAME}/contrib/electrs-fetch-cookie.sh" ]; then
  install -m 0755 "${NAME}/contrib/electrs-fetch-cookie.sh" /usr/local/bin/electrs-fetch-cookie
fi
if [ ! -x /usr/local/bin/electrs-fetch-cookie ]; then
  echo "/usr/local/bin/electrs-fetch-cookie is missing" >&2
  exit 1
fi

# Workdir (also holds the db when ELECTRS_ARGS uses --db-dir under it)
mkdir -p "${ELECTRS_WORKDIR}"
chown "${SERVICE_USER}:${SERVICE_USER}" "${ELECTRS_WORKDIR}"

# Static environment file
mkdir -p "${ENV_DIR}"
if [ -n "${NEW_ENV}" ]; then
  umask 077
  printf '%s' "${NEW_ENV}" > "${ENV_DIR}/env.new"
  echo "RUST_BACKTRACE=1" >> "${ENV_DIR}/env.new"
  mv "${ENV_DIR}/env.new" "${ENV_DIR}/env"
fi
if [ ! -f "${ENV_DIR}/env" ]; then
  echo "${ENV_DIR}/env does not exist; pass it on stdin" >&2
  exit 1
fi
chmod 600 "${ENV_DIR}/env"
chown root:root "${ENV_DIR}/env"

# systemd unit from the release
export SERVICE_USER ELECTRS_WORKDIR
envsubst '$SERVICE_USER $ELECTRS_WORKDIR' < "${NAME}/contrib/electrs.service" > "/etc/systemd/system/${SERVICE_NAME}.service"
sed -i "/^ProtectHome=/a ReadWritePaths=${ELECTRS_WORKDIR}" "/etc/systemd/system/${SERVICE_NAME}.service"

systemctl daemon-reload
systemctl enable "${SERVICE_NAME}"
systemctl restart "${SERVICE_NAME}"
sleep 3
systemctl is-active "${SERVICE_NAME}"
