# GitHub Actions CI/CD Pipeline

This document describes the GitHub Actions workflows configured for the mTLS Proxy project.

## Overview

The project uses several GitHub Actions workflows to ensure code quality, run tests, build installation assets, and manage releases:

- **CI/CD Pipeline** (`ci.yml`) - Main workflow for testing and building
- **Docker** (`docker.yml`) - Docker image building and publishing
- **Code Quality** (`code-quality.yml`) - Linting, formatting, and security checks
- **Dependencies** (`dependencies.yml`) - Dependency management and updates

## Workflow Details

### 1. CI/CD Pipeline (`ci.yml`)

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` branch
- Release creation

**Jobs:**

#### Test Job
- **Platforms:** Ubuntu, macOS, Windows
- **Rust versions:** Stable, 1.75
- **Actions:**
  - Install Rust toolchain
  - Cache dependencies
  - Install system dependencies
  - Generate test certificates
  - Run unit tests
  - Run integration tests
  - Run performance tests
  - Run security tests
  - Check code formatting
  - Run clippy linting

#### Security Job
- **Dependencies:** Requires test job completion
- **Actions:**
  - Run `cargo audit` for security vulnerabilities
  - Run `cargo deny` for dependency policy enforcement
  - Run Trivy vulnerability scanner
  - Upload security scan results

#### Build Job
- **Dependencies:** Requires test and security job completion
- **Triggers:** Only on push to main or release creation
- **Actions:**
  - Build release binary
  - Build Docker image
  - Build RPM package
  - Create deployment files (Kubernetes, Docker Compose)
  - Create installation script
  - Upload build artifacts

#### Release Job
- **Dependencies:** Requires build job completion
- **Triggers:** Only on release creation
- **Actions:**
  - Download build artifacts
  - Create GitHub release with assets
  - Generate release notes

#### Documentation Job
- **Dependencies:** Requires test job completion
- **Actions:**
  - Build Rust documentation
  - Build markdown documentation (if configured)
  - Upload documentation artifacts

### 2. Docker Workflow (`docker.yml`)

**Triggers:**
- Push to `main` branch
- Tag pushes (v*)
- Pull requests to `main` branch

**Jobs:**

#### Build Job
- **Actions:**
  - Set up Docker Buildx for multi-platform builds
  - Log in to GitHub Container Registry
  - Extract metadata for tagging
  - Build and push Docker images
  - Test Docker image functionality

#### Security Job
- **Dependencies:** Requires build job completion
- **Actions:**
  - Run Trivy vulnerability scanner on Docker image
  - Upload security scan results

#### Deploy Staging Job
- **Dependencies:** Requires build and security job completion
- **Triggers:** Only on push to main (not PRs)
- **Actions:**
  - Deploy to staging environment (configurable)

### 3. Code Quality Workflow (`code-quality.yml`)

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` branch

**Jobs:**

#### Lint Job
- **Actions:**
  - Check code formatting with `cargo fmt`
  - Run clippy linting
  - Check for unused dependencies

#### Complexity Job
- **Actions:**
  - Run code coverage analysis
  - Upload coverage to Codecov

#### Security Job
- **Actions:**
  - Run `cargo audit`
  - Run `cargo deny`
  - Check for outdated dependencies

#### Documentation Job
- **Actions:**
  - Check API documentation
  - Check for broken links
  - Validate README quality

#### Performance Job
- **Triggers:** Only on pull requests
- **Actions:**
  - Run performance tests
  - Check build time
  - Monitor binary size

### 4. Dependencies Workflow (`dependencies.yml`)

**Triggers:**
- Weekly schedule (Mondays at 9 AM UTC)
- Manual workflow dispatch

**Jobs:**

#### Check Dependencies Job
- **Actions:**
  - Check for outdated dependencies
  - Check for security vulnerabilities

#### Update Dependencies Job
- **Triggers:** Only on manual dispatch
- **Actions:**
  - Update Cargo.lock
  - Create pull request with updates

## Configuration Files

### `deny.toml`
Cargo-deny configuration for:
- Security advisory checking
- License compliance
- Dependency policy enforcement

**Key settings:**
- **Allowed licenses:** MIT, Apache-2.0, BSD variants, ISC, CC0-1.0, Unlicense, Zlib, OpenSSL
- **Denied licenses:** All GPL, AGPL, and LGPL variants
- **Security:** Deny vulnerabilities, warn on yanked crates

## Installation Assets

The CI/CD pipeline creates the following installation assets:

### 1. Binary Distribution
- **Location:** `target/release/mtls-proxy`
- **Platforms:** Linux, macOS, Windows
- **Architectures:** x86_64, ARM64 (where supported)

### 2. Docker Image
- **Registry:** GitHub Container Registry (ghcr.io)
- **Tags:** 
  - `latest` (main branch)
  - `v{version}` (release tags)
  - `{branch}-{sha}` (feature branches)
- **Platforms:** linux/amd64, linux/arm64

### 3. RPM Package
- **Format:** Red Hat Package Manager
- **Dependencies:** systemd, openssl
- **Installation:** `sudo rpm -i mtls-proxy-{version}.rpm`

### 4. Deployment Files
- **Kubernetes:** `build/k8s-deployment.yaml`
- **Docker Compose:** `build/docker-compose.yml`
- **Installation Script:** `build/install.sh`

## Usage

### For Developers

1. **Local Development:**
   ```bash
   # Run tests locally
   cargo test
   
   # Check formatting
   cargo fmt --all
   
   # Run clippy
   cargo clippy --all-targets --all-features -- -D warnings
   ```

2. **Creating Pull Requests:**
   - Ensure all tests pass locally
   - Check that code formatting is correct
   - Verify no clippy warnings
   - The CI pipeline will run automatically

3. **Manual Workflow Triggers:**
   - Go to Actions tab in GitHub
   - Select "Dependencies" workflow
   - Click "Run workflow" to update dependencies

### For Maintainers

1. **Creating Releases:**
   ```bash
   # Create and push a tag
   git tag v1.0.0
   git push origin v1.0.0
   
   # Create release on GitHub
   # The CI pipeline will automatically build and upload assets
   ```

2. **Monitoring Builds:**
   - Check Actions tab for build status
   - Review security scan results
   - Monitor performance metrics

3. **Deployment:**
   - Staging deployment happens automatically on main branch pushes
   - Production deployment requires manual release creation

## Security Features

### 1. Vulnerability Scanning
- **Cargo Audit:** Checks for known vulnerabilities in dependencies
- **Trivy:** Scans Docker images and filesystem for vulnerabilities
- **SARIF Integration:** Results uploaded to GitHub Security tab

### 2. Dependency Management
- **Cargo Deny:** Enforces dependency policies
- **License Compliance:** Ensures only approved licenses are used
- **Outdated Dependencies:** Regular checks for updates

### 3. Code Quality
- **Clippy:** Rust-specific linting
- **Formatting:** Consistent code style
- **Documentation:** API documentation requirements

## Troubleshooting

### Common Issues

1. **Build Failures:**
   - Check Rust version compatibility
   - Verify system dependencies are installed
   - Review error logs in Actions tab

2. **Test Failures:**
   - Ensure test certificates are generated
   - Check for port conflicts
   - Verify test environment setup

3. **Security Scan Failures:**
   - Review vulnerability reports
   - Update dependencies if needed
   - Consider adding exceptions to `deny.toml`

4. **Docker Build Issues:**
   - Check Dockerfile syntax
   - Verify multi-platform build support
   - Review registry authentication

### Debugging

1. **Local Reproduction:**
   ```bash
   # Install the same Rust version
   rustup install 1.75
   rustup default 1.75
   
   # Install system dependencies
   sudo apt-get install pkg-config libssl-dev sqlite3
   
   # Run the same commands as CI
   cargo test --all-features
   ```

2. **Artifact Inspection:**
   - Download build artifacts from Actions tab
   - Inspect logs and output files
   - Test generated binaries locally

## Customization

### Adding New Jobs

1. **Create a new workflow file:**
   ```yaml
   name: Custom Workflow
   on: [push, pull_request]
   
   jobs:
     custom-job:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v4
         # Add your steps here
   ```

2. **Modify existing workflows:**
   - Add new steps to existing jobs
   - Create new jobs within existing workflows
   - Update triggers and conditions

### Environment Variables

Set repository secrets for:
- `DOCKER_REGISTRY`: Custom Docker registry
- `DEPLOY_KEY`: SSH key for deployment
- `CODECOV_TOKEN`: Code coverage reporting

### Conditional Execution

Use GitHub Actions expressions for conditional execution:
```yaml
if: github.ref == 'refs/heads/main'
if: github.event_name == 'release'
if: contains(github.event.head_commit.message, '[skip ci]')
```

## Best Practices

1. **Keep Workflows Fast:**
   - Use caching for dependencies
   - Parallelize independent jobs
   - Skip unnecessary steps

2. **Security First:**
   - Always run security scans
   - Keep dependencies updated
   - Use minimal base images

3. **Reliability:**
   - Add retry logic for flaky tests
   - Use timeouts for long-running jobs
   - Implement proper error handling

4. **Maintainability:**
   - Use reusable workflows
   - Document complex steps
   - Keep workflows modular

## Support

For issues with the CI/CD pipeline:
1. Check the Actions tab for detailed logs
2. Review this documentation
3. Create an issue with the workflow name and error details
4. Include relevant logs and reproduction steps
