# TODO: Convert Proxy Server from HTTPS to HTTP

## Overview
Convert the mTLS proxy server to use HTTP (port 8440) by default while maintaining HTTPS capability. Move the mock server to port 8444 to avoid port conflicts. The mock server remains on HTTPS for proper mTLS testing.

## Port Strategy
- **8440**: Default HTTP port for proxy server
- **8443**: HTTPS port for proxy server (when TLS is explicitly enabled)
- **8444**: HTTPS port for mock server

## 📋 **Remaining Tasks**

### Testing and Validation

#### 12. Update Integration Tests
- [ ] **File: `tests/integration_test.rs`**
  - [ ] Update test configurations to use new ports
  - [ ] Add tests for both HTTP and HTTPS proxy modes
  - [ ] Verify mTLS functionality with mock server on port 8444

#### 13. Update Performance Tests
- [ ] **File: `tests/performance_test.rs`**
  - [ ] Update port configurations
  - [ ] Test performance in both HTTP and HTTPS modes

#### 14. Update Security Tests
- [ ] **File: `tests/security_test.rs`**
  - [ ] Update port configurations
  - [ ] Ensure security tests work in both modes

### Documentation Updates

#### 16. Update Developer Documentation
- [ ] **File: `docs/DEVELOPER_GUIDE.md`**
  - [ ] Document new configuration options
  - [ ] Update development setup instructions
  - [ ] Explain HTTP vs HTTPS modes
- [ ] **File: `docs/LOCAL_TESTING.md`**
  - [ ] Update testing instructions with new ports
  - [ ] Document how to test both HTTP and HTTPS modes

#### 17. Update Deployment Documentation
- [ ] **File: `docs/DEPLOYMENT_PLAN.md`**
  - [ ] Update deployment configurations
  - [ ] Document port requirements
- [ ] **File: `docs/DEPLOYMENT_SUMMARY.md`**
  - [ ] Update with new port strategy

#### 18. Update API Documentation
- [ ] **File: `docs/API_DOCUMENTATION.md`**
  - [ ] Update endpoint URLs to reflect new ports
  - [ ] Document TLS configuration options

### Advanced Features (Low Priority)

#### 19. Monitoring & Observability (3-4 hours)
- [ ] **Enhanced metrics**
  - Custom business metrics
  - Certificate expiration monitoring
  - Configuration change metrics
  - Performance metrics

- [ ] **Logging improvements**
  - Structured logging
  - Log levels configuration
  - Log rotation policies
  - Log analysis tools

#### 20. Advanced Configuration (2-3 hours)
- [ ] **Configuration templates**
  - Pre-built configuration templates
  - Environment-specific configurations
  - Configuration validation rules
  - Configuration backup/restore

#### 21. UI Enhancements (2-3 hours)
- [ ] **UI improvements**
  - Real-time configuration validation
  - Certificate preview functionality
  - Configuration diff viewer
  - Better responsive design

## 📚 **Completed Tasks Documentation**

All completed tasks have been moved to: **[docs/HTTPS_TO_HTTP_CONVERSION.md](docs/HTTPS_TO_HTTP_CONVERSION.md)**

This includes:
- ✅ Configuration changes
- ✅ CLI interface updates
- ✅ Proxy server implementation
- ✅ Test script updates
- ✅ User documentation updates
- ✅ Example file updates
- ✅ Build and run script updates
- ✅ Configuration file updates
- ✅ Docker configuration updates
- ✅ Functional testing
- ✅ Integration testing
- ✅ Performance validation
- ✅ Post-implementation tasks
- ✅ Test script reorganization

## 🚨 **Known Issues & Limitations**

### Technical Limitations
1. **Certificate Upload**: Warp framework body consumption issue
   - **Workaround**: Use file-based upload or manual file placement
   - **Impact**: Medium (affects one endpoint)

2. **Authentication**: Removed for development simplicity
   - **Impact**: Low (intentional for development environment)
   - **Future**: Can be re-added if needed for production

### Performance Considerations
1. **Memory Usage**: Monitor for large request handling
2. **Concurrent Connections**: Test with high load
3. **SQLite Performance**: Consider connection pooling for high traffic

## 🎯 **Future Considerations**

### Dioxus Migration (Future Phase)
- **Phase 1**: Dioxus migration setup
- **Phase 2**: Component migration
- **Phase 3-6**: Complete full migration

## Notes
- The mock server must remain on HTTPS for proper mTLS testing
- The proxy server's mTLS client functionality (outgoing connections) remains unchanged
- All existing functionality should work in both HTTP and HTTPS modes
- Port 8440 was chosen to avoid conflicts with common development ports
- Port 8444 was chosen to maintain logical port numbering with the proxy server
- Authentication was removed for development simplicity but can be re-added if needed
- Certificate upload has a known Warp limitation with a file-based workaround available
