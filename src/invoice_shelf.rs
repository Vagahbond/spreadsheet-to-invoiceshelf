pub struct Invoice {
    date: String,
    due_date: String,
    customer_id: i32,
    invoice_number: String,
    exchange_rate: i32,
    discount_type: String,
    discount: f32,
    discount_val: f32,
    sub_total: f32,
    total: f32,
    tax: f32,
    template_name: String,
    items: Vec<InvoiceItem>,
}

pub struct InvoiceItem {
    name: String,
    quantity: i64,
    price: f64,
    description: String,
    item_id: i64,
    sub_total: f64,
    total: f64,
    unit_name: String,
    discount: f64,
    discount_type: String,
    discount_val: f64,
}
