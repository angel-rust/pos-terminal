use dioxus::prelude::*;
use crate::models::{Product, ProductCategory};

#[component]
pub fn ProductManager(
    products: Signal<Vec<Product>>,
    on_add: EventHandler<Product>,
    on_delete: EventHandler<String>,
) -> Element {
    let mut show_add_form = use_signal(|| false);
    let mut new_name = use_signal(|| String::new());
    let mut new_price = use_signal(|| String::new());
    let mut new_category = use_signal(|| ProductCategory::Food);

    let all_products = products.read();

    rsx! {
        div {
            class: "product-manager-container",

            div {
                class: "manager-header",
                h2 { "Product Management" }
                button {
                    class: "btn-primary",
                    onclick: move |_| show_add_form.set(!show_add_form()),
                    if show_add_form() { "Cancel" } else { "+ Add Product" }
                }
            }

            if show_add_form() {
                div {
                    class: "add-product-form",

                    div {
                        class: "form-group",
                        label { "Product Name" }
                        input {
                            r#type: "text",
                            class: "form-input",
                            value: "{new_name}",
                            oninput: move |e| new_name.set(e.value()),
                            placeholder: "Enter product name",
                        }
                    }

                    div {
                        class: "form-group",
                        label { "Price" }
                        input {
                            r#type: "number",
                            class: "form-input",
                            value: "{new_price}",
                            oninput: move |e| new_price.set(e.value()),
                            placeholder: "0.00",
                            step: "0.01",
                        }
                    }

                    div {
                        class: "form-group",
                        label { "Category" }
                        select {
                            class: "form-select",
                            onchange: move |e| {
                                let cat = match e.value().as_str() {
                                    "Food" => ProductCategory::Food,
                                    "Beverage" => ProductCategory::Beverage,
                                    "Retail" => ProductCategory::Retail,
                                    "Service" => ProductCategory::Service,
                                    _ => ProductCategory::Food,
                                };
                                new_category.set(cat);
                            },

                            option { value: "Food", "Food" }
                            option { value: "Beverage", "Beverage" }
                            option { value: "Retail", "Retail" }
                            option { value: "Service", "Service" }
                        }
                    }

                    button {
                        class: "btn-primary",
                        onclick: move |_| {
                            if !new_name().is_empty() && !new_price().is_empty() {
                                if let Ok(price) = new_price().parse::<f64>() {
                                    let product = Product::new(
                                        new_name(),
                                        price,
                                        new_category(),
                                    );
                                    on_add.call(product);
                                    new_name.set(String::new());
                                    new_price.set(String::new());
                                    show_add_form.set(false);
                                }
                            }
                        },
                        "Add Product"
                    }
                }
            }

            div {
                class: "products-table",
                table {
                    thead {
                        tr {
                            th { "Name" }
                            th { "Price" }
                            th { "Category" }
                            th { "Stock" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for product in all_products.iter() {
                            tr {
                                key: "{product.id}",
                                td { "{product.name}" }
                                td { "${product.price:.2}" }
                                td {
                                    span {
                                        class: "category-badge",
                                        style: "background-color: {product.category.color()};",
                                        "{product.category.as_str()}"
                                    }
                                }
                                td { "{product.stock}" }
                                td {
                                    button {
                                        class: "btn-danger-small",
                                        onclick: {
                                            let id = product.id.clone();
                                            move |_| on_delete.call(id.clone())
                                        },
                                        "Delete"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
