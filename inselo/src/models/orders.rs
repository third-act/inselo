use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderRequest {}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateOrderResponse {
    order_id: String,
    order_number: String,
    goods_owner_id: String,
    order_status: String,
    custom_object: String,
    goods_owner_order_id: String,
    stockroom: String,
    order_lines: Vec<OrderLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLine {
    /// The article number of the article that should be sent to consignee.
    /// The article number must match an article number in the registry of article numbers,
    /// else the order will be rejected.
    pub article_number: String,

    /// The number of articles of the specified article number that should be sent to consignee.
    /// The number must be an integer without decimals and must be greater than 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_items: Option<String>,

    /// The row number of the order line. Must be unique.
    /// If you do not provide row number, it will be automatically generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_number: Option<String>,

    /// Optional comment for the order line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Indicates whether the item should be picked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_be_picked: Option<bool>,

    /// Serial number for the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,

    /// Total customs value for the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_total_customs_value: Option<f64>,

    /// Batch number for the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<String>,

    /// Customer's article number reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_article_number: Option<String>,

    /// Special warehouse instructions for this line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_instruction: Option<String>,

    /// External ID reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    /// Minimum days until expiry date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_days_to_expiry_date: Option<i32>,

    /// Maximum days until expiry date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_days_to_expiry_date: Option<i32>,

    /// Free text fields for additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_free_values: Option<LineFreeValues>,
}

/// Container for free text values associated with an order line.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineFreeValues {
    /// Free text field 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_text1: Option<String>,
    // Add other free text fields as needed
}
