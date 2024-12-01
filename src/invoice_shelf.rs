use crate::template_mapping::ComputedMappingOutput;

pub struct Invoice {
    date: String,
    due_date: String,
    customer_id: i32,
    invoice_number: String,
    exchange_rate: f32,
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

impl Invoice {
    pub fn from_generated_items(items: Vec<ComputedMappingOutput>) -> Self {
        let invoice_items = items
            .iter()
            .map(|i| InvoiceItem {
                name: i.name.clone(),
                quantity: i.quantity,
                price: i.price,
                description: i.description.clone(),
                item_id: 1,
                sub_total: i.sub_total,
                total: i.total,
                unit_name: i.unit_name.clone(),
                discount: 0.0,
                discount_type: "".into(),
                discount_val: 0.0,
            })
            .collect();

        return Self {
            date: chrono::offset::Local::now().to_rfc3339(),
            due_date: chrono::offset::Local::now().to_rfc3339(),
            customer_id: 1,
            invoice_number: "a".into(),
            exchange_rate: 1.0,
            discount_type: "".into(),
            discount: 0.0,
            discount_val: 0.0,
            sub_total: 0.0,
            total: 0.0,
            tax: 0.0,
            template_name: "".into(),
            items: invoice_items,
        };
    }
}

impl InvoiceItem {
    fn new(
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
    ) -> Self {
        return Self {
            name,
            quantity,
            price,
            description,
            item_id,
            sub_total,
            total,
            unit_name,
            discount,
            discount_type,
            discount_val,
        };
    }
}
