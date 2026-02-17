# Release Process for rtop

This document describes the complete process for creating a new release and running all GitHub Actions workflows.

## Current Version

**Version**: 3.0.2

## Release Checklist

### 1. Pre-Release Verification

Before creating a release, ensure:

- [ ] All code changes are merged to `master` branch
- [ ] Version number in `Cargo.toml` is correct (currently `3.0.2`)
- [ ] `CHANGELOG.md` has an entry for the new version
- [ ] `.github/CREATE_RELEASE.md` is updated with correct version
- [ ] All tests pass locally: `cargo test`
- [ ] Code builds successfully: `cargo build --release`
- [ ] Code is properly formatted: `cargo fmt --check`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`

### 2. Creating the Release

#### Option A: Via GitHub Web Interface (Recommended)

1. Go to <https://github.com/npequeux/rtop/releases/new>

2. **Create a new tag**:
   - Click "Choose a tag"
   - Type `v3.0.2` (or the appropriate version)
   - Select "Create new tag: v3.0.2 on publish"

3. **Set Release Details**:
   - **Release title**: `v3.0.2 - Advanced Monitoring & Enhanced UI`
   - **Description**: Copy the content from `.github/CREATE_RELEASE.md`

4. **Options**:
   - ✅ Check "Set as the latest release"
   - ❌ Uncheck "Set as a pre-release" (unless this is a beta/RC)

5. Click **"Publish release"**

#### Option B: Via Git Command Line

```bash
# Ensure you're on master and up to date
git checkout master
git pull origin master

# Create and push the tag
git tag -a v3.0.2 -m "Release v3.0.2 - Advanced Monitoring & Enhanced UI"
git push origin v3.0.2

# Then go to GitHub to create the release from the tag
```

### 3. Workflows That Will Be Triggered

Once the tag is pushed, the following workflows will automatically run:

#### A. **Release Workflow** (`release.yml`)

**Triggered by**: Tag push matching `v*`

**What it does**:
- Builds release binaries for multiple platforms:
  - Linux (x86_64, aarch64, armv7)
  - macOS (x86_64, aarch64/Apple Silicon)
  - Windows (x86_64)
- Creates compressed archives for each platform
- Uploads all binaries to the GitHub Release
- Uses native builds and cross-compilation where needed

**Expected duration**: ~15-20 minutes

**Verification**:
- Check <https://github.com/npequeux/rtop/actions/workflows/release.yml>
- Verify all 6 build jobs complete successfully
- Confirm binaries are attached to the release

#### B. **Docker Workflow** (`docker.yml`)

**Triggered by**: Tag push matching `v*`

**What it does**:
- Builds multi-architecture Docker images (amd64, arm64)
- Tags images with:
  - Specific version (e.g., `3.0.2`)
  - Major.Minor version (e.g., `3.0`)
  - Major version (e.g., `3`)
  - Commit SHA
- Pushes to GitHub Container Registry (ghcr.io)

**Expected duration**: ~10-15 minutes

**Verification**:
- Check <https://github.com/npequeux/rtop/actions/workflows/docker.yml>
- Verify image is pushed to <https://github.com/npequeux/rtop/pkgs/container/rtop>
- Test pulling the image: `docker pull ghcr.io/npequeux/rtop:3.0.2`

### 4. Additional Workflows (Master Branch)

When the PR is merged to `master`, these workflows will also run:

#### C. **Rust Workflow** (`rust.yml`)

**Triggered by**: Push to `master`

**What it does**:
- Runs all tests
- Performs security audit with `cargo-audit`
- Lints markdown files
- Checks code formatting
- Runs clippy linter
- Generates code coverage reports
- Builds documentation and deploys to GitHub Pages
- Creates build artifacts for all platforms
- Generates changelog preview with git-cliff

**Expected duration**: ~20-30 minutes

**Verification**:
- Check <https://github.com/npequeux/rtop/actions/workflows/rust.yml>
- Verify all jobs pass
- Check documentation at <https://npequeux.github.io/rtop/rtop/>
- Review coverage at <https://codecov.io/gh/npequeux/rtop>

#### D. **Performance Workflow** (`performance.yml`)

**Triggered by**: Push to `master`

**What it does**:
- Builds release binary
- Measures binary size (before and after stripping)
- Tests startup time
- Measures memory usage
- Generates performance metrics report
- Deploys benchmark results to GitHub Pages

**Expected duration**: ~5-10 minutes

**Verification**:
- Check <https://github.com/npequeux/rtop/actions/workflows/performance.yml>
- Review performance metrics in workflow summary
- View historical trends at <https://npequeux.github.io/rtop/dev/bench/>

### 5. Manual Workflows

These workflows can be triggered manually via workflow_dispatch:

#### E. **Nightly Workflow** (`nightly.yml`)

**Purpose**: Create nightly development builds

**To trigger manually**:
1. Go to <https://github.com/npequeux/rtop/actions/workflows/nightly.yml>
2. Click "Run workflow"
3. Select `master` branch
4. Click "Run workflow"

**What it does**:
- Builds release binaries for all platforms
- Creates a special "nightly" pre-release
- Updates the nightly release with latest builds
- Marks build with commit SHA and timestamp

### 6. Post-Release Verification

After all workflows complete successfully:

- [ ] Verify release page: <https://github.com/npequeux/rtop/releases/latest>
- [ ] Check all binary artifacts are present (6 total)
- [ ] Confirm Docker image is available
- [ ] Test installation from release binaries
- [ ] Verify documentation is updated
- [ ] Check that badges in README show correct version
- [ ] Announce release (if applicable)

### 7. Testing the Release

#### Download and test binary:

```bash
# Linux x86_64
wget https://github.com/npequeux/rtop/releases/download/v3.0.2/rtop-linux-x86_64.tar.gz
tar xzf rtop-linux-x86_64.tar.gz
./rtop --version
./rtop --help
```

#### Test Docker image:

```bash
docker pull ghcr.io/npequeux/rtop:3.0.2
docker run --rm ghcr.io/npequeux/rtop:3.0.2 --version
```

#### Build from source:

```bash
git clone --depth 1 --branch v3.0.2 https://github.com/npequeux/rtop.git
cd rtop
cargo build --release
./target/release/rtop --version
```

## Troubleshooting

### Release workflow fails

- Check that all platform-specific dependencies are available
- For cross-compilation issues, verify the `cross` tool is working
- Review build logs for specific error messages

### Docker workflow fails

- Ensure Dockerfile.rust is present and valid
- Check that base image is accessible
- Verify multi-arch build capabilities

### Artifacts missing from release

- Confirm all build jobs completed successfully
- Check that the `create-release` job ran after builds
- Verify file paths in upload steps match actual output

### Version mismatch issues

- Ensure Cargo.toml version matches the tag
- Update CHANGELOG.md with the correct version
- Regenerate Cargo.lock if needed

## Release Rollback

If a release needs to be rolled back:

1. Mark the release as "pre-release" or delete it
2. Delete the tag: `git push --delete origin v3.0.2`
3. Fix the issues
4. Create a new patch version (e.g., v3.0.3)

## Next Release

For the next release, update:

1. Version in `Cargo.toml`
2. Create new section in `CHANGELOG.md`
3. Update `.github/CREATE_RELEASE.md` with new version number
4. Update this file (`RELEASE_PROCESS.md`) with new version number
5. Follow the release checklist above
