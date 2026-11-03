# Deploying to DigitalOcean (testnet)

Releases are deployed to DigitalOcean droplets by `.github/workflows/deploy-do.yml`,
called from `release-binaries.yml` for every release tag (prereleases included) with the
`testnet` GitHub environment. It can also be run manually via `workflow_dispatch`.

The runner SSHes in as the service user `opcat` with a per-droplet CD key and runs
`sudo /usr/local/sbin/electrs-deploy <tag>`, which downloads the release, verifies
`SHA256SUMS`, installs the binary and systemd unit, writes `/etc/electrs/env` from stdin
and restarts the service. The CD key can run that one script as root and nothing else.

No host addresses are kept in the repo. Public addresses (`DO_HOST_LIST`) and VPC
addresses (`ELECTRS_ARGS`) are environment secrets, so they are masked in the public
Actions logs (variables are printed in clear in each step's env header).

## One-time droplet bootstrap (as root)

```bash
# deploy script (root-owned; the CD user may only run this)
install -m 0755 -o root -g root contrib/electrs-deploy-do.sh /usr/local/sbin/electrs-deploy
# cookie helper (later releases ship it in the tarball and overwrite it)
install -m 0755 -o root -g root contrib/electrs-fetch-cookie.sh /usr/local/bin/electrs-fetch-cookie
echo 'opcat ALL=(root) NOPASSWD: /usr/local/sbin/electrs-deploy' > /etc/sudoers.d/electrs-deploy
chmod 440 /etc/sudoers.d/electrs-deploy && visudo -cf /etc/sudoers.d/electrs-deploy
# CD public key
cat <cd-key>.pub >> /home/opcat/.ssh/authorized_keys
```

The CD key is generated locally (`ssh-keygen -t ed25519`), one per droplet; the private
half only goes into the GitHub environment secret.

## GitHub environment

Fill a copy of `scripts/testnet.env.example` outside the repo, then:

```bash
scripts/create-testnet-env.sh --env-file <path>
```

| Name | Kind | Notes |
|---|---|---|
| `DO_HOST_LIST` | secret | comma-separated public addresses |
| `DO_KNOWN_HOSTS` | secret | pinned host keys (`ssh-keyscan`) |
| `DO_SSH_KEY` | secret | CD private key |
| `LAYER_RPC_PASSWORD` | secret | node RPC password for electrs |
| `LAYER_RPC_USER` | secret | node RPC user |
| `ELECTRS_USER` | variable | default `opcat` |
| `ELECTRS_ARGS` | secret | bind electrum/http to the VPC address only |
