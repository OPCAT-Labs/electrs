#!/bin/bash

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Electrs Multi-Arch Docker Build Script${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Check if in git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}Error: Not in a git repository${NC}"
    exit 1
fi

# Get commit ID
COMMIT_ID=$(git rev-parse --short HEAD)
echo -e "${YELLOW}Current commit ID: ${COMMIT_ID}${NC}"
echo ""

# Check for uncommitted changes
if [[ -n $(git status -s) ]]; then
    echo -e "${YELLOW}Warning: Uncommitted changes detected${NC}"
    git status -s
    echo ""
    read -p "Continue building? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${RED}Build cancelled${NC}"
        exit 1
    fi
fi

# Parse command line arguments
PLATFORMS="linux/amd64,linux/arm64"
PUSH_IMAGE="true"

while [[ $# -gt 0 ]]; do
    case $1 in
        --platform)
            PLATFORMS="$2"
            shift 2
            ;;
        --no-push)
            PUSH_IMAGE="false"
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --platform PLATFORMS   Comma-separated list of platforms (default: linux/amd64,linux/arm64)"
            echo "  --no-push             Build locally without pushing to registry"
            echo "  --help                Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                              # Build and push both amd64 and arm64"
            echo "  $0 --platform linux/amd64       # Build and push only amd64"
            echo "  $0 --no-push                    # Build locally without pushing"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Image name
REMOTE_IMAGE="ghcr.io/opcat-labs/electrs:${COMMIT_ID}"

echo -e "${GREEN}Configuration:${NC}"
echo -e "  Platforms: ${YELLOW}${PLATFORMS}${NC}"
echo -e "  Image: ${YELLOW}${REMOTE_IMAGE}${NC}"
echo -e "  Push: ${YELLOW}${PUSH_IMAGE}${NC}"
echo ""

# Ensure buildx is available
if ! docker buildx version > /dev/null 2>&1; then
    echo -e "${RED}Error: docker buildx is not available${NC}"
    echo -e "Please install Docker Buildx: https://docs.docker.com/buildx/working-with-buildx/"
    exit 1
fi

# Create buildx builder if it doesn't exist
BUILDER_NAME="electrs-builder"
if ! docker buildx ls | grep -q "${BUILDER_NAME}"; then
    echo -e "${YELLOW}Creating buildx builder: ${BUILDER_NAME}${NC}"
    docker buildx create --name "${BUILDER_NAME}" --use
    echo ""
fi

# Use the builder
docker buildx use "${BUILDER_NAME}"

echo -e "${GREEN}Building Docker image(s)${NC}"
echo ""

# Prepare build arguments
BUILD_ARGS=(
    "--platform" "${PLATFORMS}"
    "--tag" "${REMOTE_IMAGE}"
    "--build-arg" "commitHash=${COMMIT_ID}"
    "--progress" "plain"
)

# Add output type based on push flag
if [[ "${PUSH_IMAGE}" == "true" ]]; then
    BUILD_ARGS+=("--output" "type=registry")
    echo -e "${YELLOW}Will push to registry after build${NC}"
else
    BUILD_ARGS+=("--load")
    echo -e "${YELLOW}Building for local use only (--load)${NC}"
    # Note: --load only works with single platform
    if [[ "${PLATFORMS}" == *","* ]]; then
        echo -e "${RED}Error: --no-push (--load) only works with a single platform${NC}"
        echo -e "Please specify a single platform with --platform, e.g.:"
        echo -e "  $0 --no-push --platform linux/amd64"
        exit 1
    fi
fi

echo ""

# Build the image
if docker buildx build "${BUILD_ARGS[@]}" .; then
    echo ""
    echo -e "${GREEN}✓ Build successful${NC}"
    echo ""
else
    echo ""
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Build Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "Commit ID: ${YELLOW}${COMMIT_ID}${NC}"
echo -e "Image: ${YELLOW}${REMOTE_IMAGE}${NC}"
echo -e "Platforms: ${YELLOW}${PLATFORMS}${NC}"
echo ""

if [[ "${PUSH_IMAGE}" == "true" ]]; then
    echo -e "${GREEN}Image pushed to registry${NC}"
    echo -e "To pull this image:"
    echo -e "  ${YELLOW}docker pull ${REMOTE_IMAGE}${NC}"
else
    echo -e "${GREEN}Image loaded locally${NC}"
    echo -e "To run this image:"
    echo -e "  ${YELLOW}docker run ${REMOTE_IMAGE}${NC}"
fi
echo ""
