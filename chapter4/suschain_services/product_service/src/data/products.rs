use crate::data::get_connection;
use crate::models::{NewProduct, Product};
use crate::schema;
use crate::schema::products::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use schema::products;

pub fn create_product<'a>(product: &'a Product) -> Result<Product, Error> {
    let connection = get_connection();

    let new_product = NewProduct {
        product_name: product.product_name.clone(),
        product_type: product.product_type.clone(),
        amount: product.amount,
    };

    diesel::insert_into(products::table)
        .values(&new_product)
        .get_result(&connection)
}

pub fn update_product<'a>(product: &'a Product) -> Result<Product, Error> {
    let connection = get_connection();

    diesel::update(products)
        .set(product)
        .get_result::<Product>(&connection)
}

pub fn delete_product<'a>(product: &'a Product) -> Result<usize, Error> {
    let connection = get_connection();

    diesel::delete(products.find(product.id)).execute(&connection)
}

pub fn show_products() -> Result<Vec<Product>, Error> {
    let connection = get_connection();
    products.limit(10).load::<Product>(&connection)
}
