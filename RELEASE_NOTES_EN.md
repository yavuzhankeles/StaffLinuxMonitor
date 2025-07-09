# Release Notes

## v1.0.2 (2024-03-19)
- Background (daemon) support added
- Logging system added
- Error handling improved
- PID file support

### New Features
- Background operation
- Detailed logging
- Error logs
- PID file management
- Secure daemon startup

### Technical Improvements
- Log4rs integration
- Daemonize support
- Error catching and logging
- Security improvements

## v1.0.1 (2024-03-19)
- API integration added
- REST API support
- JSON format data exchange
- HTTP/HTTPS support

### New Features
- API endpoints
- Data sending/receiving
- API authentication
- Rate limiting
- Error handling

### Technical Improvements
- API client library
- HTTP request management
- JSON serialization/deserialization
- API response processing
- Connection management

## v1.0.0 (2024-03-19)
- First stable release
- Basic system monitoring features added
- Automatic updates every 2 seconds
- JSON format reporting
- GitHub Actions automatic compilation
- Static binary support (musl)

### New Features
- CPU usage monitoring
- Memory usage monitoring
- Disk usage monitoring
- Network interface monitoring
- Service status monitoring
- Security status monitoring
- Hardware information
- System load monitoring
- User access monitoring
- Automatic JSON reporting

### Technical Improvements
- GitHub Actions integration
- Static binary compilation (musl)
- CI/CD pipeline improvements
- Performance optimizations
- Error handling improvements

### Known Issues
- CPU temperature information may not be available on some systems
- Some service statuses may not be available depending on system configuration

### Future Plans
- Web interface development
- Email notifications
- Telegram/Discord integration
- More detailed reporting
- Graphical interface
- Docker support
- Windows support 