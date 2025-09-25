#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TAG=${1:-"dev"}
IMAGE_NAME="hyperb1iss/git-iris:${TAG}"
DOCKERFILE="${SCRIPT_DIR}/Dockerfile"

echo -e "${BLUE}🔨 Building Git-Iris Docker image: ${YELLOW}${IMAGE_NAME}${NC}"
echo

# Build the Docker image
docker build -t "${IMAGE_NAME}" -f "${DOCKERFILE}" ..
BUILD_STATUS=$?

if [ $BUILD_STATUS -ne 0 ]; then
  echo -e "\n${RED}❌ Build failed! See above for errors.${NC}"
  exit 1
fi

echo -e "\n${GREEN}✅ Build successful!${NC}"
echo -e "${GREEN}🚀 Image is built: ${YELLOW}${IMAGE_NAME}${NC}"

echo
echo -e "${BLUE}To run the image:${NC}"
echo -e "  docker run --rm --user \$(id -u):\$(id -g) -v \"\$(pwd):/git-repo\" ${IMAGE_NAME}"
echo
echo -e "${BLUE}To test the image:${NC}"
echo -e "  ${SCRIPT_DIR}/test-image.sh ${TAG}"
echo
echo -e "${BLUE}To push to Docker Hub:${NC}"
echo -e "  docker push ${IMAGE_NAME}"
