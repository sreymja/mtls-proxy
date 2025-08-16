# GitHub Actions Quick Start Guide

This guide will help you quickly set up and use the GitHub Actions CI/CD pipeline for the mTLS Proxy project.

## Prerequisites

1. **GitHub Repository:** Ensure your code is in a GitHub repository
2. **Repository Permissions:** You need admin access to configure workflows
3. **GitHub Actions:** Enabled in your repository settings

## Setup Steps

### 1. Enable GitHub Actions

1. Go to your repository on GitHub
2. Click on the **Settings** tab
3. Scroll down to **Actions** in the left sidebar
4. Select **General**
5. Under "Actions permissions", choose **Allow all actions and reusable workflows**
6. Click **Save**

### 2. Configure Repository Secrets (Optional)

For advanced features, set up these secrets in **Settings > Secrets and variables > Actions**:

- `DOCKER_REGISTRY`: Custom Docker registry URL
- `DEPLOY_KEY`: SSH key for deployment
- `CODECOV_TOKEN`: Code coverage reporting token

### 3. Push the Workflow Files

The workflow files are already in your repository:
- `.github/workflows/ci.yml` - Main CI/CD pipeline
- `.github/workflows/docker.yml` - Docker image building
- `.github/workflows/code-quality.yml` - Code quality checks
- `.github/workflows/dependencies.yml` - Dependency management

### 4. Create Your First Release

To test the complete pipeline:

```bash
# Create and push a tag
git tag v0.1.0
git push origin v0.1.0

# Create a release on GitHub
# Go to Releases > Create a new release
# Select the tag v0.1.0
# The CI pipeline will automatically build and upload assets
```

## What Happens Next

### On Every Push to Main Branch

1. **Tests Run:** Unit, integration, performance, and security tests
2. **Code Quality Checks:** Formatting, linting, and documentation
3. **Security Scans:** Vulnerability scanning and dependency analysis
4. **Build Assets:** Docker images, RPM packages, and deployment files
5. **Docker Publishing:** Images pushed to GitHub Container Registry

### On Pull Requests

1. **Tests Run:** All tests on multiple platforms
2. **Code Quality:** Formatting and linting checks
3. **Security:** Basic security scanning
4. **Performance:** Performance regression testing

### On Release Creation

1. **Build Assets:** All installation assets created
2. **Release Upload:** Assets attached to GitHub release
3. **Documentation:** Release notes generated automatically

## Monitoring Your Workflows

### View Workflow Status

1. Go to your repository on GitHub
2. Click on the **Actions** tab
3. You'll see all workflow runs with their status

### Check Specific Jobs

1. Click on a workflow run
2. Click on a specific job to see detailed logs
3. Download artifacts if needed

### Common Status Indicators

- ✅ **Green:** All checks passed
- ❌ **Red:** One or more checks failed
- 🟡 **Yellow:** Workflow is running
- ⚠️ **Orange:** Some checks failed but workflow continued

## Troubleshooting Common Issues

### Workflow Not Running

**Problem:** Workflows don't start when you push code

**Solution:**
1. Check that GitHub Actions is enabled in repository settings
2. Verify the workflow files are in `.github/workflows/`
3. Check the workflow trigger conditions in the YAML files

### Test Failures

**Problem:** Tests fail in CI but pass locally

**Solution:**
1. Check the test logs for specific error messages
2. Ensure test certificates are generated (CI does this automatically)
3. Verify system dependencies are installed
4. Check for platform-specific issues

### Lock File Version Issues

**Problem:** `lock file version '4' was found, but this version of Cargo does not understand this lock file`

**Solution:**
1. The CI pipeline now automatically handles this issue
2. Run `./scripts/check-rust-version.sh` locally to fix lock file issues
3. Update your local Rust version: `rustup update`
4. Regenerate lock file: `rm Cargo.lock && cargo generate-lockfile`

### Build Failures

**Problem:** Build job fails

**Solution:**
1. Check Rust version compatibility
2. Verify all dependencies are available
3. Check for missing system packages
4. Review the build logs for specific errors

### Security Scan Failures

**Problem:** Security scans report vulnerabilities

**Solution:**
1. Review the vulnerability reports
2. Update dependencies if needed
3. Add exceptions to `deny.toml` if necessary
4. Consider using alternative packages

## Quick Commands

### Local Testing (Before Pushing)

```bash
# Run all tests
cargo test

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Check for security issues
cargo audit

# Build release binary
cargo build --release
```

### Manual Workflow Triggers

1. Go to **Actions** tab
2. Select a workflow (e.g., "Dependencies")
3. Click **Run workflow**
4. Choose branch and options
5. Click **Run workflow**

### Creating Releases

```bash
# Create a new version tag
git tag v1.0.0

# Push the tag
git push origin v1.0.0

# Create release on GitHub (web interface)
# Assets will be automatically attached
```

## Next Steps

### Customization

1. **Modify Workflows:** Edit the YAML files to add custom steps
2. **Add Environments:** Configure staging/production environments
3. **Custom Notifications:** Add Slack, email, or other notifications
4. **Performance Monitoring:** Add performance regression testing

### Advanced Features

1. **Multi-Platform Builds:** Already configured for Linux, macOS, Windows
2. **Docker Multi-Arch:** Images built for AMD64 and ARM64
3. **Security Scanning:** Integrated vulnerability scanning
4. **Dependency Management:** Automated dependency updates

### Integration

1. **External Services:** Connect to external monitoring services
2. **Deployment:** Configure automatic deployment to staging/production
3. **Notifications:** Set up alerts for build failures
4. **Metrics:** Track build times and success rates

## Getting Help

### Documentation

- **Full Documentation:** See `docs/GITHUB_ACTIONS.md`
- **Workflow Files:** Check the YAML files in `.github/workflows/`
- **Configuration:** Review `deny.toml` for dependency policies

### Support

1. **GitHub Issues:** Create an issue with workflow name and error details
2. **GitHub Actions Documentation:** [docs.github.com/en/actions](https://docs.github.com/en/actions)
3. **Community:** Check GitHub Discussions or community forums

### Debugging

1. **Local Reproduction:** Run the same commands locally
2. **Log Analysis:** Check detailed logs in the Actions tab
3. **Artifact Inspection:** Download and examine build artifacts
4. **Environment Comparison:** Compare local and CI environments

## Success Checklist

- [ ] GitHub Actions enabled in repository settings
- [ ] Workflow files pushed to `.github/workflows/`
- [ ] First push triggers workflow execution
- [ ] All tests pass on multiple platforms
- [ ] Security scans complete successfully
- [ ] Build assets are generated
- [ ] Docker images are published
- [ ] Release creation works with assets
- [ ] Documentation is up to date

Congratulations! Your mTLS Proxy project now has a comprehensive CI/CD pipeline that will automatically test, build, and release your software.
