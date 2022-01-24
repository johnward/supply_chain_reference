use serde_derive::{Deserialize, Serialize};

pub enum ProductType {
    mp3,
    book,
    mp4,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    product_name: String,
    media_type: Option<String>,
    gtin: String,
}

fn get_product(product_name: String) {}

fn new_product(product: Product) {}
