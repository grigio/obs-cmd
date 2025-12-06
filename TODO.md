# TODO: obs-cmd Improvements and Missing Features

## Analysis Summary

Based on comparison with the OBS WebSocket v5 specification, the current obs-cmd implementation covers approximately 40% of available requests. This document outlines missing features and suggested improvements.

## Priority Levels

- **High**: Core functionality that significantly expands tool capabilities
- **Medium**: Important features for complete OBS control
- **Low**: Nice-to-have features and edge cases

## 🚨 High Priority Missing Features


### 3. Advanced Input Management
- **Input Creation/Removal**: `CreateInput`, `RemoveInput`, `SetInputName`
- **Input Settings**: `GetInputSettings`, `SetInputSettings`, `GetInputDefaultSettings`
- **Input Properties**: `GetInputPropertiesListPropertyItems`, `PressInputPropertiesButton`
- **Audio Controls**: Volume, balance, sync offset, monitor type, audio tracks


## 📋 Medium Priority Missing Features


### 4. Media Input Enhancements
- **Media Status**: `GetMediaInputStatus`
- **Media Actions**: Next, previous playlist items
- **Media Properties**: Duration, cursor position control

### 5. UI and Projectors
- **Dialog Control**: Open input properties, filters, interact dialogs
- **Advanced Projectors**: Video mix projectors, custom geometry
- **Monitor Management**: `GetMonitorList`

## 🔧 Low Priority Missing Features


### 3. Monitoring and Statistics
- **System Stats**: Enhanced `GetStats` with detailed performance metrics
- **Source Active State**: Check if sources are active/showing
- **Input Volume Meters**: High-volume audio monitoring

## 🏗️ Architectural Improvements

### 2. CLI Enhancements
- **Command Completion**: Shell integration for auto-completion
- **Configuration Files**: Support for default connection settings
- **Output Formatting**: JSON, table, or custom output formats
- **Batch Operations**: Execute multiple commands in sequence


## 🧪 Testing and Quality

### 1. Test Coverage
- **Unit Tests**: Comprehensive handler testing
- **Integration Tests**: End-to-end command testing
- **Mock Server**: Test against OBS WebSocket mock
- **Error Scenarios**: Test failure modes and edge cases

### 2. Documentation
- **Command Reference**: Complete command documentation
- **Examples**: Real-world usage examples
- **Migration Guide**: Upgrading from previous versions
- **Troubleshooting**: Common issues and solutions

### 3. Code Quality
- **Type Safety**: Enhanced type definitions
- **Code Organization**: Better module structure
- **Dependency Updates**: Keep dependencies current
- **Security**: Audit for security vulnerabilities

## 📦 Feature Parity Matrix

| Category | Current Coverage | Missing Features | Priority |
|----------|------------------|------------------|----------|
| General | 80% | Stats, custom events, vendor requests | Medium |
| Config | 10% | Scene collections, profiles, video settings | High |
| Sources | 30% | Screenshots, active state, creation/removal | High |
| Scenes | 40% | Creation/removal, transitions, studio mode | High |
| Inputs | 20% | Creation/removal, settings, audio controls | High |
| Transitions | 0% | All transition features | Medium |
| Filters | 10% | Basic filter operations | Medium |
| Scene Items | 95% | Private settings, source lookup | Low |
| Outputs | 60% | Virtual cam, replay buffer (missing generic) | Medium |
| Stream | 70% | Basic control, missing captioning | Low |
| Record | 80% | Basic control, missing chapters/split | Low |
| Media | 50% | Basic playback, missing status/properties | Medium |
| UI | 30% | Projectors, missing dialogs/monitors | Medium |



## 📝 Notes

- The current implementation focuses on the most commonly used OBS operations
- Many missing features are specialized use cases or advanced configurations
- Some features (like T-Bar control) are marked as deprecated in the spec
- Consider user feedback and actual usage patterns when prioritizing
- Maintain backward compatibility when adding new features

This analysis provides a comprehensive roadmap for expanding obs-cmd to cover the full OBS WebSocket v5 specification while maintaining the tool's simplicity and ease of use.