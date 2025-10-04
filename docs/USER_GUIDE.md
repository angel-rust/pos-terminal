# Trezza Terminal User Guide

## Getting Started

Trezza Terminal is a modern Point of Sale system designed for simplicity and efficiency.

## Interface Overview

### Header
- **TREZZA TERMINAL**: Application branding
- **MENU**: Opens the sidebar navigation

### Sidebar Navigation
Access the sidebar by clicking the MENU button in the header.

- **POS**: Main point of sale interface
- **History**: View completed orders
- **Products**: Manage product catalog
- **Light/Dark Mode**: Toggle theme preference

## Features

### 1. Point of Sale (POS)

#### Adding Items to Cart
1. Browse products in the grid view
2. Use category filters to find items:
   - All
   - Beverage
   - Food
   - Retail
   - Service
3. Click a product to add it to the cart

#### Managing Cart
- **Quantity**: Use + and - buttons to adjust quantities
- **Remove**: Click remove button to delete an item
- **Total**: View real-time cart total at the bottom
- **Clear Cart**: Remove all items at once
- **Checkout**: Proceed to payment

#### Processing Payment
1. Click "Checkout" button
2. Review order summary
3. Select payment method:
   - Cash
   - Card
   - Mobile
4. Click "Complete Payment" to finalize
5. Order is saved to history

### 2. Order History

View all completed transactions:
- Order ID and timestamp
- Items purchased with quantities
- Payment method used
- Total amount

### 3. Product Management

#### Adding Products
1. Fill in the form:
   - Name: Product name
   - Price: Decimal format (e.g., 9.99)
   - Category: Select from dropdown
2. Click "Add Product"

#### Removing Products
- Click "Delete" button next to any product in the table
- Product is immediately removed

## Keyboard Shortcuts

Currently, all interactions are mouse/touch-based.

## Tips

### Efficient Workflow
1. Keep frequently used categories visible
2. Use the category filter to quickly find items
3. Adjust quantities in cart before checkout
4. Review order summary before completing payment

### Dark Mode
- Default theme is dark mode
- Toggle to light mode via sidebar menu
- Setting persists during session

### Empty States
- Empty cart shows "Your cart is empty"
- No order history shows "No orders yet"
- All states are clearly indicated

## Troubleshooting

### Application Won't Start
- Verify Rust is installed (`rustc --version`)
- Run `cargo build` to check for errors
- Check console for error messages

### Items Not Adding to Cart
- Ensure you're clicking the product card
- Check if product has valid price
- Try refreshing by clicking POS view again

### Payment Not Processing
- Verify cart has items
- Ensure payment method is selected
- Check if modal is fully loaded

## Data Persistence

**Note**: Current version does not persist data between sessions. All orders and products are cleared when the application closes.

Future versions will include:
- Local database storage
- Export/import functionality
- Backup and restore features

## Support

For issues or questions, please refer to the project repository or documentation.
