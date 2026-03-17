# Changelog

## 0.2.0 - 2026-03-15

### Added

- Added MAX_URI_LENGTH as an environment variable to limit expected length to redirect to HTTPS.

### Changed

- Max URI length is now enforced by default for https redirection. Current default is set to 2048 characters, and can be extended further if needed via the MAX_URI_LENGTH environment variable.
- HTTPS Redirection now uses the HTTPS_REDIRECT_HOST environment variable to determine the domain name to redirect to rather than the URL.