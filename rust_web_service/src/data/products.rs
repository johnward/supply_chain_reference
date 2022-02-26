use crate::data::get_connection;
use crate::models::{NewProduct, Product};
use crate::schema;
use crate::schema::products::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use schema::products;

pub fn create_product<'a>(conn: &PgConnection, product: &'a Product) -> Result<Product, Error> {
    let new_product = NewProduct {
        product_name: product.product_name.clone(),
        product_type: product.product_type.clone(),
        amount: product.amount,
    };

    let product_result = diesel::insert_into(products::table)
        .values(&new_product)
        .get_result(conn);

    // Debug return type
    println!("Ret: {:?}", product_result.as_ref());

    product_result
}

pub fn update_product<'a>(con: &PgConnection, product: &'a Product) -> Result<Product, Error> {
    let product_result = diesel::update(products)
        .set(product)
        .get_result::<Product>(con);

    // Debug return type
    println!("Published post {:?}", product_result.as_ref());

    product_result
}

pub fn delete_product<'a>(con: &PgConnection, product: &'a Product) -> Result<usize, Error> {
    let num_deleted = diesel::delete(products.find(product.id)).execute(con);

    // Debug return type
    println!("Deleted {:?} posts", num_deleted);

    num_deleted
}

pub fn show_products() -> Result<Vec<Product>, Error> {
    let connection = get_connection();
    let results = products.limit(10).load::<Product>(&connection);

    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.product_name);
    //     println!("----------\n");
    //     println!("{}", post.address);
    // }

    results
}
