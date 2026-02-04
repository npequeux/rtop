# CI/CD Documentation

## Overview

This project uses GitHub Actions for continuous integration, deployment, and automation. The workflows are optimized for speed, reliability, and comprehensive testing.

## Workflows

### 1. Main CI (`rust.yml`)

**Triggers:**

- Push to master (excludes markdown files)
- Pull requests (excludes markdown files)
- Manual dispatch with custom Rust version

**Features:**

- **Concurrency Control**: Cancels old runs when new commits are pushed
- **Build Caching**: Uses sccache for 3-5x faster Rust compilation
- **Multi-version Testing**: Tests on Rust 1.88.0, stable, and beta
- **Linting**: cargo fmt and clippy with strict warnings
- **Code Coverage**: Generates coverage reports and uploads to Codecov
- **Security Audit**: Scans dependencies for vulnerabilities
- **Markdown Linting**: Validates all documentation
- **Cross-platform Testing**: Ubuntu, macOS, Windows
- **Multi-architecture Builds**: x86_64 and ARM64 for Linux/macOS

**Jobs:**

1. `test` - Build and test on multiple Rust versions
2. `lint_format` - Format and lint checking
3. `coverage` - Code coverage analysis
4. `security` - Dependency security audit
5. `markdown_lint` - Documentation linting
6. `docs` - Documentation generation
7. `cross` - Cross-platform testing
8. `artifacts` - Multi-arch binary builds
9. `deploy` - Production deployment (master only)

### 2. Release Automation (`release.yml`)

**Triggers:**

- Git tags matching `v*` (e.g., v3.0.0)

**Features:**

- Builds release binaries for 5 platforms
- Creates GitHub Release automatically
- Uploads all binaries as release assets
- Generates release notes from commits

**Platforms:**

- `linux-x86_64` - Linux Intel/AMD 64-bit
- `linux-aarch64` - Linux ARM64 (Raspberry Pi, AWS Graviton)
- `macos-x86_64` - macOS Intel
- `macos-aarch64` - macOS Apple Silicon (M1/M2/M3)
- `windows-x86_64` - Windows 64-bit

**Usage:**

```bash
# Create and push a tag
git tag -a v3.1.0 -m "Release v3.1.0"
git push origin v3.1.0

# GitHub Actions will:
# 1. Build all platform binaries
# 2. Create a GitHub Release
# 3. Upload binaries as release assets
```

### 3. Docker Build (`docker.yml`)

**Triggers:**

- Push to master
- Git tags
- Pull requests
- Manual dispatch

**Features:**

- Multi-architecture builds (amd64, arm64)
- Pushes to GitHub Container Registry (ghcr.io)
- Layer caching for fast rebuilds
- Semantic versioning tags
- Optimized multi-stage Dockerfile

**Image Tags:**

- `latest` - Latest master build
- `v3.0.0` - Specific version
- `v3.0` - Major.minor version
- `v3` - Major version
- `sha-abc123` - Git commit SHA

**Usage:**

```bash
# Pull and run
docker pull ghcr.io/npequeux/rtop:latest
docker run -it --rm --pid=host --privileged ghcr.io/npequeux/rtop:latest

# Using docker-compose
docker compose up
```

### 4. Performance Tracking (`performance.yml`)

**Triggers:**

- Push to master
- Pull requests
- Manual dispatch

**Features:**

- Measures binary size
- Tracks build times
- Monitors startup performance
- Measures memory usage
- Comments metrics on PRs

**Metrics Collected:**

- Binary size (before/after stripping)
- Build time
- Startup time (5 iterations)
- Maximum resident memory
- Average resident memory

### 5. Changelog Generation (`changelog` job in rust.yml)

**Features:**

- Uses git-cliff for automatic changelog generation
- Includes only source code changes
- Uploads preview as artifact

## Caching Strategy

### Cargo Registry Cache

```yaml
- uses: actions/cache@v4
  with:
    path: ~/.cargo/registry
    key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
```

### Sccache (Compilation Cache)

```yaml
- uses: mozilla-actions/sccache-action@v0.0.4
```

**Benefits:**

- 3-5x faster builds on cache hit
- Shared across workflow runs
- Reduces GitHub Actions minutes usage

## Security

### Dependency Scanning

- `cargo audit` runs on every commit
- Known issues are documented in `.cargo/audit.toml`
- Fails CI on new vulnerabilities

### Ignored Advisories

Current known issues (waiting for upstream fixes):

- RUSTSEC-2021-0119: Nix vulnerability (battery dependency)
- RUSTSEC-2020-0168: Mach unmaintained (battery dependency)
- RUSTSEC-2024-0436: Paste unmaintained (ratatui dependency)
- RUSTSEC-2026-0002: LRU unsound (ratatui dependency)

## Code Coverage

- Coverage reports uploaded to [Codecov](https://codecov.io/gh/npequeux/rtop)
- Minimum coverage target: 70%
- PR comments show coverage changes
- Badge in README shows current coverage

**Setup Required:**

Add `CODECOV_TOKEN` secret in GitHub repository settings.

## Best Practices

### Making a Release

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit changes: `git commit -m "chore: Release v3.1.0"`
4. Create tag: `git tag -a v3.1.0 -m "Release v3.1.0"`
5. Push: `git push origin master --tags`
6. GitHub Actions creates the release automatically

### Pull Request Workflow

1. Create feature branch: `git checkout -b feature/amazing-feature`
2. Make changes and commit
3. Push: `git push origin feature/amazing-feature`
4. Create PR on GitHub
5. CI runs automatically:
   - Tests on multiple Rust versions
   - Linting and formatting checks
   - Coverage analysis
   - Performance metrics (commented on PR)
   - Security audit
   - Markdown linting

### Skipping CI

Add to commit message to skip workflows:

```bash
git commit -m "docs: Update README [skip ci]"
```

Or modify only markdown files (already filtered).

## GitHub Secrets Required

### For Full Functionality

- `CODECOV_TOKEN` - Codecov upload token (optional but recommended)
- `GITHUB_TOKEN` - Automatically provided by GitHub Actions

### Not Required

- Docker Registry: Uses GitHub Container Registry (automatic)
- Release creation: Uses GitHub API (automatic)

## Troubleshooting

### Build Failures

1. Check the specific job that failed
2. Review error logs in GitHub Actions
3. Test locally with same Rust version:
   ```bash
   rustup install 1.88.0
   cargo +1.88.0 build --verbose
   ```

### Cache Issues

Clear cache by:

1. Incrementing cache key in workflow
2. Or manually delete in GitHub Actions cache settings

### Docker Build Failures

Test locally:

```bash
docker build -f Dockerfile.rust -t rtop:test .
docker run -it --rm rtop:test
```

## Performance Tips

### Reduce CI Time

- Push to feature branches for testing
- Use draft PRs for work-in-progress
- Skip CI for doc-only changes

### Local Development

Use the fast release profile:

```bash
cargo build --profile release-fast
```

## Monitoring

- **Build Status**: Check [Actions tab](https://github.com/npequeux/rtop/actions)
- **Coverage**: [Codecov Dashboard](https://codecov.io/gh/npequeux/rtop)
- **Security**: [Dependabot Alerts](https://github.com/npequeux/rtop/security/dependabot)
- **Docker Images**: [Packages](https://github.com/npequeux/rtop/pkgs/container/rtop)
