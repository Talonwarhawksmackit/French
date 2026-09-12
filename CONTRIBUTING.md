# Contributing to French

Thank you for your interest in contributing to French! This document provides guidelines for submitting contributions.

## Code of Conduct

Be respectful and constructive in all interactions. We're building a welcoming community.

## Getting Started

1. Fork the repository
2. Clone your fork locally
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Install dependencies: `npm install && cargo build`

## Development Setup

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Ollama (for testing)

### Building and Testing

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Run tests
npm test
cargo test

# Lint code
npm run lint

# Type check
npm run type-check
```

## Contribution Types

### Bug Reports

1. Check existing issues to avoid duplicates
2. Provide a clear, descriptive title
3. Include steps to reproduce
4. Describe expected vs. actual behavior
5. Include environment details (Windows version, Ollama version, etc.)

### Feature Requests

1. Search existing issues first
2. Describe the use case and benefits
3. Provide examples if applicable
4. Discuss implementation approach if you have ideas

### Code Contributions

1. Fork and create a feature branch
2. Make focused, atomic commits
3. Add tests for new functionality
4. Update documentation
5. Submit a pull request with clear description

## Pull Request Guidelines

### Before Submitting

- [ ] Code follows project style guidelines
- [ ] Tests pass: `npm test && cargo test`
- [ ] No linting errors: `npm run lint`
- [ ] TypeScript types are correct: `npm run type-check`
- [ ] Documentation is updated
- [ ] Commit messages are clear and descriptive

### PR Description

Include:
- What problem does this solve?
- How does it work?
- Any breaking changes?
- Testing instructions
- Screenshots (if UI changes)

### Review Process

1. Maintainers review your PR
2. Address any feedback
3. Ensure CI passes
4. Merge when approved

## Code Style

### Rust

- Format with `rustfmt`
- Lint with `clippy`
- Follow Rust naming conventions
- Add comments for complex logic

```bash
cargo fmt
cargo clippy
```

### TypeScript/React

- Use ESLint configuration provided
- Follow React best practices
- Use TypeScript for type safety
- Add JSDoc comments for exported functions

```bash
npm run lint -- --fix
```

## Commit Messages

Write clear, descriptive commit messages:

```
[Type] Brief description

Optional detailed explanation. Explain the why, not the what.

Fixes #123
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Example:
```
fix: prevent path traversal in filesystem sandbox

Validate parent directories before creating new files to prevent
attempts to escape the sandbox using relative paths.

Fixes #456
```

## Documentation

Update relevant documentation:
- **README.md** — For user-facing changes
- **docs/ARCHITECTURE.md** — For structural changes
- **docs/SECURITY.md** — For security-related changes
- **Code comments** — For complex logic

## Testing

### Adding Tests

1. Unit tests for individual functions
2. Integration tests for workflows
3. Security tests for sensitive operations

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_validation() {
        // Test code here
    }
}
```

```typescript
import { describe, it, expect } from 'vitest';

describe('MyComponent', () => {
  it('should render correctly', () => {
    // Test code here
  });
});
```

## Performance Considerations

- Minimize frontend-backend IPC calls
- Use async/await for non-blocking operations
- Cache frequently accessed data
- Monitor performance impact of changes

## Security

- Never commit credentials or secrets
- Use parameterized queries for database access
- Validate all user input
- Follow secure coding practices
- Report security issues privately (see README)

## Questions?

- Open an issue for discussion
- Check existing documentation
- Review similar PRs for patterns

---

Thank you for contributing to French! 🎉
