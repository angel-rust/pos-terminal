# Architecture Documentation

## Overview

Trezza Terminal is a desktop Point of Sale application built with Rust and Dioxus, featuring a reactive UI with state management through signals.

## Technology Stack

### Core Technologies
- **Rust 2021**: Systems programming language providing memory safety and performance
- **Dioxus 0.6**: React-like framework for building desktop applications
- **Dioxus Desktop**: Native desktop rendering using webview
- **Serde**: Serialization/deserialization for data structures
- **Chrono**: Date and time handling
- **UUID**: Unique identifier generation

### Architecture Pattern

The application follows a component-based architecture with unidirectional data flow:

```
┌─────────────────────────────────────────────┐
│                   App                       │
│  (Main component with global state)         │
└─────────────────┬───────────────────────────┘
                  │
    ┌─────────────┼─────────────┐
    │             │             │
    ▼             ▼             ▼
┌────────┐  ┌──────────┐  ┌──────────┐
│  POS   │  │ History  │  │ Products │
│  View  │  │   View   │  │   View   │
└───┬────┘  └──────────┘  └──────────┘
    │
    ├─────────────┬─────────────┐
    ▼             ▼             ▼
┌────────────┐ ┌──────┐ ┌─────────────┐
│ProductGrid │ │ Cart │ │PaymentModal │
└────────────┘ └──────┘ └─────────────┘
```

## State Management

### Global State (in App component)
- `products`: Vector of all available products
- `current_order`: Active shopping cart
- `order_history`: Completed orders
- `selected_category`: Current product filter
- `show_payment_modal`: Payment modal visibility
- `current_view`: Active view (POS, History, Products)
- `dark_mode`: Theme preference
- `sidebar_open`: Sidebar visibility

### State Flow
1. User interactions trigger callbacks
2. Callbacks modify signals
3. Dioxus reactively updates UI
4. Components re-render with new state

## Module Structure

### Models Layer
**Purpose**: Data structures and business logic

- `Product`: Product information and category
- `Order`: Shopping cart with items and totals
- `Payment`: Payment processing data

**Responsibilities**:
- Data validation
- Business logic (totals, item management)
- State transitions (order status)

### Components Layer
**Purpose**: Reusable UI components

- `ProductGrid`: Product display and filtering
- `Cart`: Shopping cart interface
- `PaymentModal`: Payment processing UI
- `OrderHistory`: Order history display
- `ProductManager`: Product CRUD operations

**Responsibilities**:
- User interaction handling
- Visual rendering
- Props-based configuration
- Event propagation via callbacks

### Application Layer
**Purpose**: Top-level coordination

- `main.rs`: App initialization and routing
- Global state management
- View switching logic
- Theme and layout control

## Data Flow

### Adding Product to Cart
```
ProductGrid (click)
  → on_product_click callback
  → current_order.write().add_item()
  → Cart re-renders
```

### Completing Purchase
```
Cart (checkout)
  → show_payment_modal.set(true)
  → PaymentModal renders
  → User selects payment method
  → on_complete callback
  → order.complete_payment()
  → order_history.push()
  → current_order.clear()
  → show_payment_modal.set(false)
```

### Product Management
```
ProductManager (add form)
  → Validate input
  → Create Product
  → on_add callback
  → products.write().push()
  → ProductGrid re-renders
```

## Styling Architecture

### CSS Organization
- Global resets and typography
- Layout utilities (flex, grid)
- Component-specific styles
- Dark mode overrides

### Theme System
```css
/* Light Mode (default) */
background: #f4f4f5
text: #18181b

/* Dark Mode (.dark-mode class) */
background: #0b0f19
text: #e4e4e7
accent: #2563eb, #60a5fa
```

### Responsive Design
- Flexbox and Grid for layouts
- Fixed sidebar width (320px)
- Fluid main content area
- Responsive product grid

## Performance Considerations

### Optimizations
- Signal-based reactivity (only re-render on change)
- Efficient Vec operations for collections
- CSS transitions for smooth animations
- Minimal DOM updates

### Build Profiles
- **Debug**: Fast compilation, unoptimized
- **Release**: Full optimizations, LTO enabled

## Security Considerations

- No external network calls
- Local state only
- Input validation on forms
- Type-safe Rust guarantees

## Future Architecture Considerations

### Potential Enhancements
- Persistent storage (SQLite, file-based)
- Backend API integration
- Multi-user support
- Inventory tracking
- Receipt printing
- Analytics dashboard

### Scalability
- State persistence layer
- Service layer for business logic
- Repository pattern for data access
- Event-driven architecture for complex workflows
