# Local Testing Before Push

This document explains the various options for running tests locally before pushing to ensure code quality and prevent CI failures.

## 🎯 Overview

You have several options to run the same tests that run in the CI pipeline locally before pushing:

1. **Git Hooks** (Recommended) - Automatic blocking of pushes
2. **Makefile** - Easy command-line interface
3. **Scripts** - Manual execution with flexibility
4. **VS Code Tasks** - Integrated development experience

## 🚀 Option 1: Git Hooks (Recommended)

### Setup
The pre-push hook is already installed and will automatically run before every `git push`.

### How it works
- Automatically runs when you execute `git push`
- Blocks the push if any tests fail
- Runs all CI checks: tests, formatting, linting, security audits
- Generates certificates if needed

### Usage
```bash
# Just push normally - hooks run automatically
git push origin main

# If tests fail, the push is blocked until you fix them
```

### Bypass (if needed)
```bash
# Skip the pre-push hook (use with caution!)
git push --no-verify origin main
```

## 🛠️ Option 2: Makefile

### Setup
The Makefile is already configured with all necessary targets.

### Available Commands
```bash
# Show all available commands
make help

# Setup development environment
make setup

# Run all tests
make test

# Run specific test types
make test-unit
make test-integration
make test-performance
make test-security

# Run all CI checks
make ci

# Individual checks
make format
make lint
make audit
make deny

# Clean build artifacts
make clean
```

### Usage Examples
```bash
# Quick test run
make test

# Full CI pipeline
make ci

# Just check formatting
make format
```

## 📜 Option 3: Scripts

### Setup
The test script is already executable and ready to use.

### Available Commands
```bash
# Run all tests
./scripts/run-tests.sh

# Run specific test types
./scripts/run-tests.sh unit
./scripts/run-tests.sh integration
./scripts/run-tests.sh performance
./scripts/run-tests.sh security

# Run full CI pipeline
./scripts/run-tests.sh ci
```

### Usage Examples
```bash
# Quick test run
./scripts/run-tests.sh

# Full CI pipeline
./scripts/run-tests.sh ci

# Just unit tests
./scripts/run-tests.sh unit
```

## 🔧 Option 4: VS Code Tasks

### Setup
VS Code tasks are configured in `.vscode/tasks.json`.

### Available Tasks
- **Setup Development Environment** - Install dependencies and generate certificates
- **Run All Tests** - Execute all test suites
- **Run Unit Tests** - Execute unit tests only
- **Run Integration Tests** - Execute integration tests only
- **Run Performance Tests** - Execute performance tests only
- **Run Security Tests** - Execute security tests only
- **Run Full CI Pipeline** - Execute all CI checks
- **Check Code Formatting** - Verify code formatting
- **Run Linting** - Execute clippy linting
- **Run Security Audit** - Execute cargo audit
- **Run Dependency Checks** - Execute cargo deny
- **Clean Build Artifacts** - Clean build artifacts

### Usage
1. Open VS Code
2. Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS)
3. Type "Tasks: Run Task"
4. Select the desired task

## 📋 What Each Option Runs

All options run the same set of checks that the CI pipeline runs:

### Tests
- ✅ Unit tests (`cargo test --lib`)
- ✅ Integration tests (`cargo test --test integration_test`)
- ✅ Performance tests (`cargo test --test performance_test`)
- ✅ Security tests (`cargo test --test security_test`)
- ✅ All tests with features (`cargo test --all-features`)

### Code Quality
- ✅ Code formatting (`cargo fmt --all -- --check`)
- ✅ Linting (`cargo clippy --all-targets --all-features -- -D warnings`)

### Security
- ✅ Security audit (`cargo audit`)
- ✅ Dependency checks (`cargo deny check`)

### Setup
- ✅ Certificate generation (if needed)
- ✅ Rust toolchain verification

## 🎯 Recommended Workflow

### For Daily Development
1. **Use VS Code Tasks** for quick checks during development
2. **Use Makefile** for command-line operations
3. **Rely on Git Hooks** to catch issues before push

### Before Committing
```bash
# Quick check
make test

# Or use the script
./scripts/run-tests.sh
```

### Before Pushing
```bash
# Full CI check
make ci

# Or let the git hook handle it automatically
git push origin main
```

## 🚨 Troubleshooting

### Common Issues

#### Certificate Files Missing
```bash
# Generate certificates
make setup
# or
./scripts/generate_certs.sh
```

#### Cargo Deny Not Installed
```bash
# Install cargo-deny
cargo install cargo-deny
# or
make install-deny
```

#### Git Hook Not Working
```bash
# Check if hook is executable
ls -la .git/hooks/pre-push

# Make executable if needed
chmod +x .git/hooks/pre-push
```

#### Tests Failing
1. Check certificate files exist
2. Run `cargo clean` and try again
3. Check for dependency issues with `cargo update`

### Performance Tips

#### Skip Slow Tests During Development
```bash
# Just unit tests (fastest)
make test-unit

# Unit + integration (medium)
make test-unit && make test-integration
```

#### Parallel Execution
```bash
# Run tests in parallel (if your system supports it)
cargo test --lib -- --test-threads=4
```

## 🔄 Integration with CI

The local testing setup mirrors the CI pipeline exactly:

- **Same commands** - All local commands match CI steps
- **Same environment** - Uses same Rust toolchain and dependencies
- **Same certificates** - Generates same test certificates
- **Same checks** - Runs same security and quality checks

This ensures that if tests pass locally, they will likely pass in CI.

## 📊 Monitoring and Reporting

### Test Results
- All options provide colored output for easy reading
- Clear success/failure indicators
- Detailed error messages for debugging

### Performance Metrics
- Test execution time is displayed
- Individual test suite timing available
- Memory usage can be monitored

### Security Reports
- Cargo audit provides vulnerability reports
- Cargo deny shows dependency policy violations
- Clear recommendations for fixing issues
