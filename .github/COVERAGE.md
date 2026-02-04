# Setting Up Code Coverage

## Codecov Integration

Your repository has code coverage configured but needs the Codecov token to upload reports.

### Step 1: Sign up for Codecov

1. Go to [codecov.io](https://codecov.io/)
2. Sign in with your GitHub account
3. Add your repository: `npequeux/rtop`

### Step 2: Get Your Codecov Token

1. Go to your repository on Codecov: https://codecov.io/gh/npequeux/rtop
2. Navigate to Settings → General
3. Copy your **Repository Upload Token**

### Step 3: Add Token to GitHub Secrets

1. Go to your GitHub repository: https://github.com/npequeux/rtop/settings/secrets/actions
2. Click "New repository secret"
3. Name: `CODECOV_TOKEN`
4. Value: Paste the token from Codecov
5. Click "Add secret"

### Step 4: Verify

The next CI run will automatically upload coverage reports to Codecov.

You can view coverage at: https://codecov.io/gh/npequeux/rtop

## Coverage Badge

Already added to README.md:

```markdown
[![codecov](https://codecov.io/gh/npequeux/rtop/branch/master/graph/badge.svg)](https://codecov.io/gh/npequeux/rtop)
```

## Coverage Configuration

Coverage settings are in `codecov.yml`:

- **Target**: Auto (maintains current coverage)
- **Threshold**: 5% tolerance
- **Range**: 70-100%
- **Comments**: Enabled on PRs

## Viewing Coverage Reports

### On Codecov Dashboard
- Overall coverage percentage
- Coverage by file
- Coverage trends over time
- PR coverage diffs

### As CI Artifacts
Coverage reports are also uploaded as workflow artifacts:
- Go to Actions → Select a workflow run
- Download "coverage-lcov" artifact
- View `lcov.info` locally with tools like:
  - `genhtml` (lcov)
  - VS Code Coverage Gutters extension
  - IntelliJ IDEA coverage viewer

## Local Coverage Generation

Generate coverage locally:

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Generate coverage report
cargo llvm-cov --workspace --all-features --html

# Open in browser
open target/llvm-cov/html/index.html
```

## Troubleshooting

### "Missing CODECOV_TOKEN"

If you see this warning in CI:
1. The coverage report is still generated
2. It's just not uploaded to Codecov
3. Add the token following steps above

### Low Coverage Warning

The project currently has low test coverage. To improve:

```bash
# Run tests with coverage to see which modules need tests
cargo llvm-cov --html --open
```

Focus on testing:
- Core monitoring modules (`src/monitor/*.rs`)
- Configuration parsing (`src/config.rs`)
- Export functionality (`src/export.rs`)
