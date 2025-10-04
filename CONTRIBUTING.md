# Contributing to Trezza Terminal

Thank you for your interest in contributing to Trezza Terminal.

## Development Setup

1. Install Rust (1.70 or higher)
2. Clone the repository
3. Run `cargo build` to verify setup
4. Run `cargo run` to start the application

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Run `cargo clippy` to check for common issues
- Write descriptive commit messages
- Keep functions small and focused

## Project Structure

```
pos_system/
├── src/
│   ├── models/          # Data models
│   │   ├── product.rs
│   │   ├── order.rs
│   │   └── payment.rs
│   ├── components/      # UI components
│   │   ├── product_grid.rs
│   │   ├── cart.rs
│   │   ├── payment_modal.rs
│   │   ├── order_history.rs
│   │   └── product_manager.rs
│   └── main.rs          # App entry point
├── styles.css           # Global styles
└── Cargo.toml           # Dependencies
```

## Adding New Features

1. Create a new branch for your feature
2. Implement your changes
3. Test thoroughly in both light and dark modes
4. Update documentation if needed
5. Submit a pull request

## Styling Guidelines

- Use  color palette
- Primary blue: `#2563eb`
- Dark background: `#0b0f19`
- Light background: `#f4f4f5`
- Accent blue: `#60a5fa`
- Maintain dark mode compatibility

## Testing

Currently, the project uses manual testing. When adding features:

1. Test all user interactions
2. Verify dark/light mode rendering
3. Check responsive behavior
4. Validate form inputs
5. Test edge cases (empty cart, no products, etc.)

## Pull Request Process

1. Ensure code compiles without warnings
2. Format code with `cargo fmt`
3. Run `cargo clippy` and fix issues
4. Update README.md if needed
5. Describe your changes in the PR description

## Reporting Issues

When reporting bugs, include:
- Description of the issue
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details (OS, Rust version)

## Questions?

Open an issue for discussion or clarification.
