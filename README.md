# Trezza Terminal

A modern, Zed-inspired Point of Sale system built with Rust and Dioxus.

## Features

- Clean, minimal interface with Zed Editor-inspired design
- Dark mode by default with light mode toggle
- Collapsible sidebar navigation
- Product management
- Order history tracking
- Multiple payment methods
- Real-time cart updates
- Category-based product filtering

## Architecture

The application is structured into modular components:

### Models
- `Product`: Product data model with categories
- `Order`: Order management and cart functionality
- `Payment`: Payment processing and methods

### Components
- `ProductGrid`: Display and filter products by category
- `Cart`: Shopping cart with real-time totals
- `PaymentModal`: Payment processing interface
- `OrderHistory`: View completed orders
- `ProductManager`: Add and manage products

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Installation

1. Clone the repository
2. Navigate to the project directory
3. Run the application:

```bash
cargo run
```

### Development

Build the project:
```bash
cargo build
```

Run in release mode for optimized performance:
```bash
cargo run --release
```

## Design

The UI follows Zed Editor's design principles:
- Dark theme: `#0b0f19` background with `#2563eb` blue accents
- Light theme: `#f4f4f5` background with clean typography
- Smooth transitions and modern glassmorphism effects
- Minimalist header with collapsible sidebar

## Technology Stack

- **Framework**: Dioxus 0.6
- **Language**: Rust 2021 Edition
- **UI**: CSS3 with custom styling
- **State Management**: Dioxus signals

## License

MIT License
