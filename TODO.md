# Chequer Development Roadmap

## Vertical Slice: Network Latency Test (v0.1.0)

### Phase 1: Core Communication Infrastructure
- [ ] **Issue #1**: Implement TCP server in host mode
  - Accept connections on configurable port
  - Handle client connections
  - Send/receive protocol messages
  
- [ ] **Issue #2**: Implement TCP client in client mode
  - Connect to host via IP:port
  - Send ping requests with timestamps
  - Receive pong responses
  
- [ ] **Issue #3**: Implement ping/pong protocol
  - Serialize/deserialize messages (JSON)
  - Calculate round-trip time
  - Collect latency samples

### Phase 2: Data Collection & Analysis
- [ ] **Issue #4**: Implement latency statistics calculation
  - Min/max/average latency
  - Jitter calculation (variance)
  - Packet loss detection
  
- [ ] **Issue #5**: Enhance report generation
  - Traffic light thresholds (green < 20ms, yellow < 50ms, red >= 50ms)
  - Colored terminal output
  - JSON export functionality

### Phase 3: Test Game Integration
- [ ] **Issue #6**: Add diagnostics overlay to test game
  - Display FPS and frame time
  - Show network status indicator
  - Embed timestamps in render loop
  
- [ ] **Issue #7**: Connect test game to agent
  - Optional: Send frame metrics to agent
  - Display connection status

### Phase 4: Polish & Testing
- [ ] **Issue #8**: Add error handling and logging
  - Connection failures
  - Timeout handling
  - Graceful shutdown
  
- [ ] **Issue #9**: Create integration tests
  - Test host/client communication
  - Verify report accuracy
  - Mock network conditions

- [ ] **Issue #10**: Documentation
  - Usage examples
  - Configuration options
  - Troubleshooting guide

## Future Enhancements (Post v0.1.0)

### v0.2.0: Bandwidth Testing
- [ ] Implement throughput measurement
- [ ] Add upload/download tests
- [ ] Support configurable test duration

### v0.3.0: Video/Audio Diagnostics
- [ ] Codec detection (H.264, H.265, AV1)
- [ ] Decode performance benchmarks
- [ ] Audio device enumeration
- [ ] Audio routing validation

### v0.4.0: Steam Integration
- [ ] Parse Steam Remote Play logs
- [ ] Extract session information
- [ ] Detect common misconfigurations
- [ ] Auto-remediation suggestions

### v0.5.0: Advanced Features
- [ ] WebSocket support (alternative to TCP)
- [ ] HTML report generation
- [ ] Real-time monitoring dashboard
- [ ] Historical data tracking

## Development Guidelines

### Branch Naming
- `feature/issue-N-short-description` for new features
- `bugfix/issue-N-short-description` for bug fixes
- `docs/issue-N-short-description` for documentation

### Commit Message Format
```
[#N] Short description

Detailed explanation if needed.

Closes #N
```

### Testing Checklist
- [ ] Code compiles without warnings
- [ ] Tests pass (`cargo test`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] Format checked (`cargo fmt --check`)
- [ ] Documentation updated
