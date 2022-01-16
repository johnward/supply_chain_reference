mod datastore;

struct Product {
    product_name: String,
    gtin: String,
}

fn get_product(product_name: String) -> Result<Product, Error> {
    // access the database etc and retreive the data

    Ok(Product {
        product_name: String::from("Test"),
        gtin: String::from("ABC"),
    })
}

fn new_product(product: Product) -> Result<bool, Error> {
    Ok(true)
}
