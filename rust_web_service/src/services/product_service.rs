use crate::data::*;
use crate::models::Product;

/// The endpoint to get a current list of all products
/// # Arguments
///
///            
/// # Return type
/// * Responder trait or Error
///
pub fn show_products() -> Vec<Product> {
    products::show_products()
}

/// The endpoint to create a new product
/// # Arguments
///
/// * 'product'
///            
/// # Return type
/// * Product
///
pub fn create_product<'a>(product: &'a Product) -> Product {
    // Create product business logic here,

    // Potentially search cache first.

    // Call the product service
    let product = products::create_product(&product);

    // send the product to the cache

    product
}

/// The endpoint to delete a new product
/// # Arguments
///
/// * 'product'
///            
/// # Return type
///
pub fn delete_product<'a>(product: &'a Product) {
    // Get the appropiate connection
    let connection = get_connection();

    // Now call the delete function in our data interface
    products::delete_product(&connection, &product);
}

/// The endpoint to update a new product
/// # Arguments
///
/// * 'product' -
///            
/// # Return type
///
pub fn update_product<'a>(product: &'a Product) {
    // Get the data interface connection
    let connection = get_connection();

    // Call update product data interface
    products::update_product(&connection, &product);
}
