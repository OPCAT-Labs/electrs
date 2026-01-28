# GPG Signing Setup Guide

This guide explains how to set up GPG signing for electrs releases.

## Prerequisites

- GPG installed on your system
- Access to GitHub repository settings (for adding secrets)
- Maintainer access to the repository

## Step 1: Generate GPG Key

Generate a new GPG key specifically for release signing:

```bash
gpg --full-generate-key
```

When prompted:
- **Key type**: Choose `(1) RSA and RSA`
- **Key size**: `4096` bits
- **Expiration**: `0` (does not expire) or set an appropriate expiration
- **Real name**: `OPCAT Labs Release Signing`
- **Email**: Use an appropriate email (e.g., `releases@opcat-labs.org`)
- **Comment**: `Release signing key for electrs`

## Step 2: Export Keys

After generating the key, export both public and private keys:

```bash
# List keys to get the KEY_ID
gpg --list-secret-keys --keyid-format LONG

# Export private key (for GitHub Secrets)
gpg --export-secret-keys --armor YOUR_KEY_ID > private-key.asc

# Export public key (for repository)
gpg --export --armor YOUR_KEY_ID > KEYS
```

**Important**: Keep `private-key.asc` secure and delete it after adding to GitHub Secrets.

## Step 3: Add Private Key to GitHub Secrets

1. Go to your repository on GitHub
2. Navigate to **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Name: `GPG_PRIVATE_KEY`
5. Value: Paste the entire contents of `private-key.asc`
6. Click **Add secret**

## Step 4: Commit Public Key to Repository

Add the public key to the repository so users can verify signatures:

```bash
# Copy the KEYS file to repository root
cp KEYS /path/to/electrs/

# Commit and push
git add KEYS
git commit -m "Add GPG public key for release signing"
git push
```

## Step 5: Publish Public Key to Key Servers (Optional)

Optionally publish your public key to key servers for easier verification:

```bash
gpg --keyserver keys.openpgp.org --send-keys YOUR_KEY_ID
gpg --keyserver keyserver.ubuntu.com --send-keys YOUR_KEY_ID
```

## Step 6: Test the Setup

Test that the GPG signing works correctly:

```bash
# Create a test file
echo "test" > test.txt

# Sign it
gpg --detach-sign --armor test.txt

# Verify the signature
gpg --verify test.txt.asc test.txt
```

## Verification

After setup, the GitHub Actions workflow will automatically:
1. Import the GPG private key from secrets
2. Sign the SHA256SUMS file
3. Upload SHA256SUMS.asc to the release

Users can then verify releases by:
1. Importing the public key from the KEYS file
2. Verifying the signature on SHA256SUMS

## Security Notes

- **Never commit the private key** to the repository
- Store the private key backup securely (encrypted)
- Consider using a hardware security key for additional protection
- Rotate keys periodically (every 2-3 years)
- If the key is compromised, revoke it immediately and generate a new one

## Key Information

After setup, document the key information:
- **Key ID**: (from `gpg --list-keys`)
- **Fingerprint**: (from `gpg --fingerprint`)
- **Created**: Date of key creation
- **Purpose**: Release signing for electrs binaries

This information should be added to SECURITY.md for user reference.
