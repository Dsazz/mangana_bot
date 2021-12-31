# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.3.0] - 2021-12-31
### Added
- Support for the https://mangalib.me
- Node.js CLI module for extracting html and bypassing site protection
- Introduced a new system of dependencies. The parsing logic is divided into parsers and extractors
- Unit tests
### Changed
- Dockerfile. Added new dependencies NodeJS and Chromium. Due to this changes, the system was changed from Debian to Alphine.

## [2.2.0] - 2021-12-20
### Added
- Changelog
- Readme
### Changed
- All text has been translated into Russian
- Allowed #![allow(clippy::non_ascii_literal)]

## [2.1.0] - 2021-12-20
### Added
- Release a new version