# Multi-Architecture Docker Build Guide

This document explains how to build and push multi-architecture Docker images for electrs.

## Supported Architectures

- **linux/amd64** (x86_64)
- **linux/arm64** (ARM 64-bit, Apple Silicon)

## Prerequisites

1. **Docker Buildx**: Ensure you have Docker Buildx installed
   ```bash
   docker buildx version
   ```

2. **QEMU** (for cross-compilation): Install QEMU to build ARM images on x86 hosts
   ```bash
   docker run --privileged --rm tonistiigi/binfmt --install all
   ```

3. **GitHub Container Registry Authentication**:
   ```bash
   echo $GHCR_TOKEN | docker login ghcr.io -u YOUR_USERNAME --password-stdin
   ```

## Building Multi-Arch Images

### Using the Build Script

The `scripts/build-and-push.sh` script now supports multi-architecture builds:

**Build and push both amd64 and arm64** (default):
```bash
./scripts/build-and-push.sh
```

**Build only amd64**:
```bash
./scripts/build-and-push.sh --platform linux/amd64
```

**Build only arm64**:
```bash
./scripts/build-and-push.sh --platform linux/arm64
```

**Build locally without pushing**:
```bash
# Only works with single platform
./scripts/build-and-push.sh --no-push --platform linux/amd64
```

**Show help**:
```bash
./scripts/build-and-push.sh --help
```

### Manual Build with Docker Buildx

```bash
# Get commit hash
COMMIT_ID=$(git rev-parse --short HEAD)

# Create/use builder
docker buildx create --name electrs-builder --use || docker buildx use electrs-builder

# Build and push multi-arch image
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  --tag ghcr.io/opcat-labs/electrs:${COMMIT_ID} \
  --tag ghcr.io/opcat-labs/electrs:latest \
  --build-arg commitHash=${COMMIT_ID} \
  --output type=registry \
  .
```

## GitHub Actions Workflow

The `.github/workflows/on-tag.yml` workflow automatically builds multi-arch images when you push a version tag:

```bash
git tag v3.3.4
git push origin v3.3.4
```

This will:
1. Build for both `linux/amd64` and `linux/arm64`
2. Push to `ghcr.io/opcat-labs/electrs:v3.3.4`
3. Update `ghcr.io/opcat-labs/electrs:latest`

## Verifying Multi-Arch Images

After pushing, verify the image manifest includes both architectures:

```bash
docker manifest inspect ghcr.io/opcat-labs/electrs:latest
```

You should see entries for both:
- `"architecture": "amd64", "os": "linux"`
- `"architecture": "arm64", "os": "linux"`

## Pulling Images

Docker automatically pulls the correct architecture for your platform:

```bash
# On x86_64 machine -> pulls linux/amd64
# On ARM machine (M1/M2 Mac, ARM server) -> pulls linux/arm64
docker pull ghcr.io/opcat-labs/electrs:latest
```

To explicitly pull a specific architecture:

```bash
# Force amd64 on ARM machine
docker pull --platform linux/amd64 ghcr.io/opcat-labs/electrs:latest

# Force arm64 on x86 machine
docker pull --platform linux/arm64 ghcr.io/opcat-labs/electrs:latest
```

## Troubleshooting

### "unknown/unknown" Architecture

If you see `unknown/unknown` in the registry, it means an image was pushed without proper platform metadata. This typically happens when using `docker build` instead of `docker buildx`.

**Solution**: Always use `docker buildx` with `--platform` flag:
```bash
docker buildx build --platform linux/amd64,linux/arm64 ...
```

### Build Fails for ARM64

**Common causes**:
1. **No QEMU emulation**: Install binfmt support
   ```bash
   docker run --privileged --rm tonistiigi/binfmt --install all
   ```

2. **RocksDB compilation fails**: Ensure sufficient memory (4GB+ recommended)
   - GitHub Actions: Uses swap file workaround (already configured)
   - Local: Increase Docker Desktop memory limit

3. **Slow build times**: ARM emulation on x86 is slower
   - Expect 2-3x longer build times for cross-compilation
   - Use `--cache-from` and `--cache-to` for faster rebuilds

### Testing ARM64 Images

On x86 machine with QEMU:
```bash
docker run --platform linux/arm64 --rm ghcr.io/opcat-labs/electrs:latest --version
```

On M1/M2 Mac (native):
```bash
docker run --rm ghcr.io/opcat-labs/electrs:latest --version
```

## Best Practices

1. **Always use buildx for pushes**: Never use `docker build` + `docker push` for registry images
2. **Tag with commit hash**: Enables tracking exactly what code is in each image
3. **Test both architectures**: Before releasing, test on both amd64 and arm64 if possible
4. **Clean old images**: Periodically clean up old tagged images from the registry

## References

- [Docker Buildx Documentation](https://docs.docker.com/buildx/working-with-buildx/)
- [Multi-platform images](https://docs.docker.com/build/building/multi-platform/)
- [GitHub Container Registry](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry)
