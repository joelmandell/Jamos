# Copilot Instructions for Jamos

## Project Overview

Jamos is an experimental operating system project. This is a research and development project exploring operating system concepts and implementations.

## Development Guidelines

### Code Style and Standards

- Implement proper error handling for system calls and hardware interactions
- Use RAII or similar patterns for resource management (memory, file descriptors, locks)
- Follow kernel coding style guidelines where applicable
- Maintain clear and well-documented code for complex system-level operations
- Use descriptive variable and function names
- Include comments for non-obvious implementation details, especially for low-level operations

### Architecture

- This is an experimental OS project, so expect unconventional approaches and exploratory code
- Be mindful of system-level considerations such as memory management, process scheduling, and hardware interactions
- Consider performance implications of changes at the OS level

### Testing

- When adding new features, include appropriate tests where applicable
- For low-level OS components, consider both unit tests and integration tests
- Document testing approaches for hardware-dependent features

### Documentation

- Update documentation when making significant changes
- Include rationale for design decisions, especially for experimental features
- Document any dependencies or build requirements

## Common Tasks

### Building

- Check for build scripts, Makefiles, or bootloader configurations in the repository
- Consider cross-compilation settings and target architecture specifications
- Verify toolchain requirements (assembler, linker, compiler versions)
- Document build requirements and dependencies

### Testing

- Run existing tests before making changes
- Ensure new code includes tests where feasible

## Important Notes

- This is an experimental project, so innovative and unconventional solutions are encouraged
- Balance experimental approaches with maintainable code
- Consider the educational value of implementations alongside functionality
