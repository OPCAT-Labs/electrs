#!/bin/bash

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Electrs Docker Build and Push Script${NC}"
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

# Image names
LOCAL_IMAGE="mempool-electrs:latest"
REMOTE_IMAGE="ghcr.io/opcat-labs/electrs:${COMMIT_ID}"

echo -e "${GREEN}Step 1/3: Building Docker image${NC}"
echo -e "Local image: ${LOCAL_IMAGE}"
echo ""

if docker build -t "${LOCAL_IMAGE}" .; then
    echo -e "${GREEN}✓ Build successful${NC}"
    echo ""
else
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi

echo -e "${GREEN}Step 2/3: Tagging image${NC}"
echo -e "Remote image: ${REMOTE_IMAGE}"
echo ""

if docker tag "${LOCAL_IMAGE}" "${REMOTE_IMAGE}"; then
    echo -e "${GREEN}✓ Tag successful${NC}"
    echo ""
else
    echo -e "${RED}✗ Tag failed${NC}"
    exit 1
fi

echo -e "${GREEN}Step 3/3: Pushing image to registry${NC}"
echo -e "Pushing to: ${REMOTE_IMAGE}"
echo ""

if docker push "${REMOTE_IMAGE}"; then
    echo -e "${GREEN}✓ Push successful${NC}"
    echo ""
else
    echo -e "${RED}✗ Push failed${NC}"
    exit 1
fi

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Build and Push Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "Commit ID: ${YELLOW}${COMMIT_ID}${NC}"
echo -e "Image: ${YELLOW}${REMOTE_IMAGE}${NC}"
echo ""
echo -e "To pull this image:"
echo -e "  ${YELLOW}docker pull ${REMOTE_IMAGE}${NC}"
echo ""
