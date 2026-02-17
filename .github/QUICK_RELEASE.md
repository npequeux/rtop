# 🚀 Quick Release Guide

## To Create Release v3.0.2

### Step 1: Merge This PR
Merge this PR to the `master` branch.

### Step 2: Create the Release

**Option A - GitHub Web UI (Easiest):**

1. Go to: https://github.com/npequeux/rtop/releases/new
2. Click "Choose a tag" → Type `v3.0.2` → "Create new tag: v3.0.2 on publish"
3. Release title: `v3.0.2 - Advanced Monitoring & Enhanced UI`
4. Copy description from: `.github/CREATE_RELEASE.md`
5. Check ✅ "Set as the latest release"
6. Click "Publish release"

**Option B - Command Line:**

```bash
git checkout master
git pull origin master
git tag -a v3.0.2 -m "Release v3.0.2 - Advanced Monitoring & Enhanced UI"
git push origin v3.0.2
```

Then create the release on GitHub using the tag.

### Step 3: Wait for Workflows

All GitHub Actions will automatically run:
- ✅ **release.yml** - Builds 6 platform binaries (~15-20 min)
- ✅ **docker.yml** - Multi-arch Docker images (~10-15 min)
- ✅ **rust.yml** - Tests, docs, coverage (~20-30 min)
- ✅ **performance.yml** - Benchmarks (~5-10 min)

### Step 4: Verify

Check that all workflows completed:
- https://github.com/npequeux/rtop/actions

View the release:
- https://github.com/npequeux/rtop/releases/latest

---

## 📚 Documentation

- **Detailed Release Guide:** `.github/RELEASE_PROCESS.md`
- **Release Notes Template:** `.github/CREATE_RELEASE.md`
- **Complete Summary:** `.github/RELEASE_SUMMARY.md`
- **This Quick Guide:** `.github/QUICK_RELEASE.md`

## 🔍 Verification Tool

Run anytime to check if ready for release:
```bash
./check_release.sh
```

---

**That's it! The repository is fully prepared.** 🎉
