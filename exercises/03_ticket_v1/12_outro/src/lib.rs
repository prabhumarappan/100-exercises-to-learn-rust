// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        if product_name.is_empty() {
            panic!("Product name cannot be empty");
        }
        if product_name.len() > 300 {
            panic!("Product name cannot be longer than 300 characters");
        }
        if quantity <= 0 {
            panic!("Quantity must be strictly greater than zero");
        }
        if unit_price <= 0 {
            panic!("Unit price must be strictly greater than zero");
        }

        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, new_product_name: String) {
        if new_product_name.is_empty() {
            panic!("Product name cannot be empty");
        }
        if new_product_name.len() > 300 {
            panic!("Product name cannot be longer than 300 characters");
        }
        self.product_name = new_product_name;
    }

    pub fn set_quantity(&mut self, new_quantity: u32) {
        if new_quantity <= 0 {
            panic!("Quantity must be strictly greater than zero");
        }
        self.quantity = new_quantity;
    }

    pub fn set_unit_price(&mut self, new_unit_price: u32) {
        if new_unit_price <= 0 {
            panic!("Unit price must be strictly greater than zero");
        }
        self.unit_price = new_unit_price;
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
