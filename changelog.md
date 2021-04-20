# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### To Do
- Add tests for word boundaries inside repeated groups

### Unfinished Ideas

## [0.1.5] - 2021-03-28

### Added
- Basic benchmarks
- Tests for 
    - Repetition bug
    - Capture index replacer
    - Rouch benchmark
    - Word boundaries and anchors are back!
- Various changes to keep up to date with Native Regex library    

### Changed
- Regex set tests modified to work with hash maps

## [0.1.4] - 2021-03-23

### Added
- Test for `Captures` returned by `SetMatches`
- Add tests for unicode characters
- Tests for replacer function
- Added test for empty regex
- Made use of `Captures::first` function when matching

## [0.1.3] - 2021-03-21

### Added
- Tests for NativeRegexSet
- Tests for regexes that terminate due to end of string
- Tests for multiline flags with start and end anchors

## [0.1.2] - 2021-03-18

### Added
- Tests for start and end anchors
- Tests for named captures

### Changed
- Modified tests to accommodate new trait structure

## [0.1.1] - 2021-03-17

### Added
- Several new types of regexes to test
- Tests for captures, matches and splitting, replacing, word boundaries and character classes

## [0.1.0] - 2021-03-17

Initial commit

### Added
- Build script to generate Native Regexes
