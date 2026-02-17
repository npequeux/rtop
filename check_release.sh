#!/usr/bin/env bash

# Release Readiness Check Script for rtop
# This script verifies that the repository is ready for a release

set -e

echo "========================================="
echo "rtop Release Readiness Check"
echo "========================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track overall status
READY=true

# Function to print status
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
        READY=false
    fi
}

# Get version from Cargo.toml
VERSION=$(grep -m1 '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
echo "Current version: $VERSION"
echo ""

# 1. Check git status
echo "Checking git status..."
if [ -z "$(git status --porcelain)" ]; then
    print_status 0 "Working directory is clean"
else
    print_status 1 "Working directory has uncommitted changes"
    git status --short
fi
echo ""

# 2. Check if on master branch
echo "Checking branch..."
BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$BRANCH" = "master" ]; then
    print_status 0 "On master branch"
else
    echo -e "${YELLOW}ℹ${NC} Currently on branch: $BRANCH (should be on master for release)"
fi
echo ""

# 3. Check if CHANGELOG has entry for current version
echo "Checking CHANGELOG.md..."
if grep -q "\[$VERSION\]" CHANGELOG.md; then
    print_status 0 "CHANGELOG.md has entry for version $VERSION"
else
    print_status 1 "CHANGELOG.md missing entry for version $VERSION"
fi
echo ""

# 4. Check if tag exists
echo "Checking git tags..."
if git rev-parse "v$VERSION" >/dev/null 2>&1; then
    echo -e "${YELLOW}⚠${NC} Tag v$VERSION already exists"
else
    print_status 0 "Tag v$VERSION does not exist yet (ready to create)"
fi
echo ""

# 5. Build check
echo "Running build check..."
if cargo build --release 2>&1 | tail -1 | grep -q "Finished"; then
    print_status 0 "Build successful"
else
    print_status 1 "Build failed"
fi
echo ""

# 6. Test check
echo "Running tests..."
if cargo test --quiet 2>&1 | tail -1 | grep -q "test result: ok"; then
    print_status 0 "All tests pass"
else
    print_status 1 "Tests failed"
fi
echo ""

# 7. Format check
echo "Checking code formatting..."
if cargo fmt --all -- --check >/dev/null 2>&1; then
    print_status 0 "Code is properly formatted"
else
    print_status 1 "Code formatting issues found (run: cargo fmt)"
fi
echo ""

# 8. Clippy check
echo "Running clippy..."
if cargo clippy --all-targets --all-features -- -D warnings >/dev/null 2>&1; then
    print_status 0 "No clippy warnings"
else
    print_status 1 "Clippy warnings found (run: cargo clippy --all-targets --all-features -- -D warnings)"
fi
echo ""

# 9. Check CREATE_RELEASE.md
echo "Checking CREATE_RELEASE.md..."
if grep -q "v$VERSION" .github/CREATE_RELEASE.md; then
    print_status 0 "CREATE_RELEASE.md references version $VERSION"
else
    print_status 1 "CREATE_RELEASE.md may need updating for version $VERSION"
fi
echo ""

# 10. Check for common issues
echo "Additional checks..."

# Check for TODO or FIXME in code
if grep -r "TODO\|FIXME" src/ >/dev/null 2>&1; then
    echo -e "${YELLOW}ℹ${NC} Found TODO/FIXME comments in source code"
else
    print_status 0 "No TODO/FIXME comments in source"
fi

# Check if Cargo.lock is committed
if [ -f Cargo.lock ]; then
    print_status 0 "Cargo.lock is present"
else
    print_status 1 "Cargo.lock is missing"
fi

echo ""
echo "========================================="
if [ "$READY" = true ]; then
    echo -e "${GREEN}✓ Repository is ready for release!${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. Push to remote: git push origin master"
    echo "  2. Create and push tag: git tag -a v$VERSION -m 'Release v$VERSION'"
    echo "  3.                      git push origin v$VERSION"
    echo "  4. Or create release via GitHub UI"
    echo ""
    echo "See .github/RELEASE_PROCESS.md for detailed instructions."
else
    echo -e "${RED}✗ Repository is NOT ready for release${NC}"
    echo "Please address the issues above before releasing."
fi
echo "========================================="
