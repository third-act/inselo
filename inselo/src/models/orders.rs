use serde::{Deserialize, Serialize};

use super::{common::Consignee, GoodsOwnerId, ItemCount, OrderNumber, ReferenceNumber};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRequest {
    /// Your goods owner ID in Inselos Warehouse management system.
    pub goods_owner_id: GoodsOwnerId,

    /// The value of this field has to be unique across all orders for the goods owner. If the
    /// value provided is already taken, the order will be rejected.
    pub order_number: OrderNumber,

    /// Custom internal reference number of goods owner.
    pub reference_number: ReferenceNumber,

    /// The type of order decides such factors as SLA. Order types are decided between goods owner
    /// and us.
    pub order_type: OrderType,

    /// The designated recipient of the ordered goods.
    pub consignee: Consignee,

    /// The order lines decide which goods will be sent to the consignee. There can only be one
    /// article number per order line.
    pub order_lines: Vec<OrderLine>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderResponse {
    pub order_id: Option<String>,
    pub order_number: String,
    pub goods_owner_id: Option<u32>,
    pub order_status: OrderStatus,
    pub goods_owner_order_id: Option<String>,
    pub stockroom: String,
    pub order_lines: Vec<OrderLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLine {
    /// The article number of the article that should be sent to consignee.
    /// The article number must match an article number in the registry of article numbers,
    /// else the order will be rejected.
    pub article_number: String,

    /// The number of articles of the specified article number that should be sent to consignee.
    /// The number must be an integer without decimals and must be greater than 0.
    pub number_of_items: ItemCount,

    /// Optional comment for the order line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Optional article number in the integrating system.
    pub customer_article_number: Option<String>,

    /// Special warehouse instructions for this line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_instruction: Option<String>,

    /// External ID reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatus(i32);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderType {
    pub code: OrderTypeCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderTypeCode {
    Business,
}
