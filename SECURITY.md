# Security Policy

## Supported Versions

We release patches for security vulnerabilities for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 3.3.x   | :white_check_mark: |
| < 3.3   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in electrs, please report it by emailing the maintainers directly. **Do not open a public issue.**

### What to Include

- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact
- Suggested fix (if any)

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity (critical issues prioritized)

## GPG Key Information

Release binaries are signed with our GPG key to ensure authenticity.

### Public Key

The public key is available in the `KEYS` file in this repository.

### Importing the Key

```bash
# Import from repository
curl -sL https://raw.githubusercontent.com/OPCAT-Labs/electrs/main/KEYS | gpg --import

# Or download and import manually
gpg --import KEYS
```

### Verifying Releases

1. Download the release binary, checksums, and signature:
   ```bash
   wget https://github.com/OPCAT-Labs/electrs/releases/download/v3.3.0/electrs-v3.3.0-linux-x86_64-musl.tar.gz
   wget https://github.com/OPCAT-Labs/electrs/releases/download/v3.3.0/SHA256SUMS
   wget https://github.com/OPCAT-Labs/electrs/releases/download/v3.3.0/SHA256SUMS.asc
   ```

2. Verify the checksum:
   ```bash
   sha256sum -c SHA256SUMS --ignore-missing
   ```

3. Verify the GPG signature:
   ```bash
   gpg --verify SHA256SUMS.asc SHA256SUMS
   ```

## Security Best Practices

When running electrs in production:

1. **Run as dedicated user**: Never run electrs as root
2. **Use systemd hardening**: See `contrib/electrs.service` for recommended settings
3. **Firewall configuration**: Restrict access to electrs ports
4. **Keep updated**: Subscribe to releases for security updates
5. **Monitor logs**: Watch for unusual activity

## Known Security Considerations

- **RPC Access**: Ensure Bitcoin RPC credentials are properly secured
- **Network Exposure**: Limit electrs network exposure to trusted clients
- **Database Access**: Protect the electrs database directory from unauthorized access

## Security Updates

Security updates are released as soon as possible after a vulnerability is confirmed. Updates are announced via:

- GitHub Security Advisories
- Release notes
- Git tags with security fixes

## Contact

For security-related inquiries, contact the maintainers through the repository's issue tracker (for non-sensitive matters) or via direct email for sensitive security issues.
