use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Channel {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Invoice {
    pub channel: Channel,
    pub user: InvoiceUser,
}

#[derive(Serialize, Deserialize)]
pub struct InvoiceUser {
    pub id: String,
}

#[derive(Serialize)]
pub struct Receipt {
    pub items: Vec<ReceiptItem>,
}

#[derive(Serialize)]
pub struct ReceiptItem {
    pub description: String,
    pub quantity: String,
    pub amount: ReceiptItemAmount,
    pub vat_code: i8,
}

#[derive(Serialize)]
pub struct ReceiptItemAmount {
    pub value: String,
    pub currency: String,
}
