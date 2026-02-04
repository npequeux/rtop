# GitHub Repository Settings

Complete these manual configuration steps in the GitHub web UI.

## 1. GitHub Pages (Required for Documentation)

Enable GitHub Pages to publish API documentation:

1. Go to https://github.com/npequeux/rtop/settings/pages
2. Under **Source**, select: **GitHub Actions**
3. Click **Save**

After the next workflow run, documentation will be available at:
**https://npequeux.github.io/rtop/**

## 2. Repository Topics (Recommended)

Add topics to improve discoverability:

1. Go to https://github.com/npequeux/rtop
2. Click the ⚙️ gear icon next to "About"
3. Add these topics:
   - `rust`
   - `system-monitor`
   - `tui`
   - `cli`
   - `performance`
   - `linux`
   - `macos`
   - `windows`
   - `docker`
   - `ratatui`
   - `cross-platform`
4. Click **Save changes**

## 3. Repository Description

Update the repository description:

1. Go to https://github.com/npequeux/rtop
2. Click the ⚙️ gear icon next to "About"
3. Set description: `A blazing-fast system monitor and task manager written in Rust`
4. Set website: `https://npequeux.github.io/rtop/`
5. Click **Save changes**

## 4. Branch Protection (Optional but Recommended)

Protect the master branch:

1. Go to https://github.com/npequeux/rtop/settings/branches
2. Click **Add branch protection rule**
3. Branch name pattern: `master`
4. Enable:
   - ✅ Require a pull request before merging
   - ✅ Require status checks to pass before merging
     - Add checks: `test`, `lint_format`, `security`, `markdown_lint`
   - ✅ Require branches to be up to date before merging
5. Click **Create**

## 5. Dependabot Alerts

Enable security alerts (usually enabled by default):

1. Go to https://github.com/npequeux/rtop/settings/security_analysis
2. Enable:
   - ✅ Dependabot alerts
   - ✅ Dependabot security updates

## Verification Checklist

After completing the settings:

- [ ] Documentation is published at https://npequeux.github.io/rtop/
- [ ] Repository has descriptive topics
- [ ] README badges show correct status
- [ ] Codecov badge shows coverage percentage
- [ ] Branch protection is active (optional)
- [ ] Dependabot is monitoring dependencies

## Current Status

### ✅ Completed
- Codecov token configured
- All workflows configured and passing
- Multi-platform builds (6 architectures)
- Docker images published to ghcr.io
- Nightly builds enabled
- Release automation configured

### ⚠️ Pending Manual Configuration
- Enable GitHub Pages
- Add repository topics
- Update repository description
