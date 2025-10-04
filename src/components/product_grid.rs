use dioxus::prelude::*;
use crate::models::{Product, ProductCategory};

#[component]
pub fn ProductGrid(
    products: Signal<Vec<Product>>,
    selected_category: Signal<Option<ProductCategory>>,
    on_product_click: EventHandler<Product>,
) -> Element {
    let filtered_products = use_memo(move || {
        let prods = products.read();
        match selected_category() {
            Some(cat) => prods.iter().filter(|p| p.category == cat).cloned().collect::<Vec<_>>(),
            None => prods.clone(),
        }
    });

    rsx! {
        div {
            class: "product-grid-container",

            // Category Filter
            div {
                class: "category-filter",
                button {
                    class: if selected_category().is_none() { "category-btn active" } else { "category-btn" },
                    onclick: move |_| selected_category.set(None),
                    "All"
                }
                button {
                    class: if selected_category() == Some(ProductCategory::Food) { "category-btn active" } else { "category-btn" },
                    onclick: move |_| selected_category.set(Some(ProductCategory::Food)),
                    "Food"
                }
                button {
                    class: if selected_category() == Some(ProductCategory::Beverage) { "category-btn active" } else { "category-btn" },
                    onclick: move |_| selected_category.set(Some(ProductCategory::Beverage)),
                    "Beverage"
                }
                button {
                    class: if selected_category() == Some(ProductCategory::Retail) { "category-btn active" } else { "category-btn" },
                    onclick: move |_| selected_category.set(Some(ProductCategory::Retail)),
                    "Retail"
                }
                button {
                    class: if selected_category() == Some(ProductCategory::Service) { "category-btn active" } else { "category-btn" },
                    onclick: move |_| selected_category.set(Some(ProductCategory::Service)),
                    "Service"
                }
            }

            // Products Grid
            div {
                class: "products-grid",
                for product in filtered_products().iter() {
                    ProductCard {
                        key: "{product.id}",
                        product: product.clone(),
                        on_click: move |p| on_product_click.call(p),
                    }
                }
            }
        }
    }
}

#[component]
fn ProductCard(product: Product, on_click: EventHandler<Product>) -> Element {
    let product_clone = product.clone();

    rsx! {
        div {
            class: "product-card",
            style: "border-left: 4px solid {product.category.color()};",
            onclick: move |_| on_click.call(product_clone.clone()),

            div {
                class: "product-info",
                h3 { class: "product-name", "{product.name}" }
                div {
                    class: "product-meta",
                    span { class: "product-category", "{product.category.as_str()}" }
                    span { class: "product-stock", "Stock: {product.stock}" }
                }
            }
            div {
                class: "product-price",
                "${product.price:.2}"
            }
        }
    }
}
