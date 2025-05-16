use serde::{Deserialize, Serialize};

use super::{common::Consignee, GoodsOwnerId, ItemCount, OrderNumber, ReferenceNumber};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRequest {
    /// Your goods owner ID in Inselos Warehouse management system.
    goods_owner_id: GoodsOwnerId,

    /// The value of this field has to be unique across all orders for the goods owner. If the value
    /// provided is already taken, the order will be rejected.
    order_number: OrderNumber,

    /// Custom internal reference number of goods owner.
    reference_number: ReferenceNumber,

    /// The type of order decides such factors as SLA. Order types are decided between goods owner and us.
    order_type: OrderType,

    /// The designated recipient of the ordered goods.
    consignee: Consignee,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderResponse {
    order_id: Option<String>,
    order_number: String,
    goods_owner_id: Option<String>,
    order_status: OrderStatus,
    custom_object: String,
    goods_owner_order_id: String,
    stockroom: String,
    order_lines: Vec<OrderLine>,
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
    code: OrderTypeInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderTypeInner {
    Business,
}
