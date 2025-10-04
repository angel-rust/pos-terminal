# Components Module

This module contains reusable UI components for Trezza Terminal.

## Components

### ProductGrid (`product_grid.rs`)
Displays products in a grid layout with category filtering.

**Props:**
- `products`: Signal containing vector of products
- `selected_category`: Signal for current category filter
- `on_product_click`: Callback when product is clicked

**Features:**
- Category filter buttons (All, Beverage, Food, Retail, Service)
- Responsive grid layout
- Product cards with name, price, and category
- Click to add items to cart

### Cart (`cart.rs`)
Shopping cart component with item management.

**Props:**
- `order`: Signal containing current order
- `on_checkout`: Callback when checkout is clicked
- `on_clear`: Callback when clear cart is clicked

**Features:**
- List of cart items with quantities
- Real-time total calculation
- Quantity adjustment buttons
- Remove item functionality
- Checkout and clear cart actions
- Empty cart state display

### PaymentModal (`payment_modal.rs`)
Modal dialog for payment processing.

**Props:**
- `order`: Signal containing order to pay
- `show`: Signal controlling modal visibility
- `on_complete`: Callback when payment is completed
- `on_cancel`: Callback when payment is cancelled

**Features:**
- Order summary with itemized list
- Payment method selection (Cash, Card, Mobile)
- Total amount display
- Complete and cancel actions
- Overlay background

### OrderHistory (`order_history.rs`)
Displays list of completed orders.

**Props:**
- `orders`: Signal containing vector of completed orders

**Features:**
- Scrollable list of orders
- Order details: ID, date, items, total
- Payment method display
- Empty state when no orders exist

### ProductManager (`product_manager.rs`)
Interface for managing products (add/delete).

**Props:**
- `products`: Signal containing vector of products
- `on_add`: Callback when new product is added
- `on_delete`: Callback when product is deleted

**Features:**
- Add product form with name, price, and category
- Product table with all items
- Delete product functionality
- Form validation
- Category-based color coding

## Usage

```rust
use components::{ProductGrid, Cart, PaymentModal};

rsx! {
    ProductGrid {
        products: products_signal,
        selected_category: category_signal,
        on_product_click: move |product| {
            // Handle product click
        }
    }

    Cart {
        order: order_signal,
        on_checkout: move |_| {
            // Handle checkout
        },
        on_clear: move |_| {
            // Handle clear cart
        }
    }
}
```

## Styling

All components use CSS classes defined in `styles.css` with support for dark mode via the `.dark-mode` class modifier.
