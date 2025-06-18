use chrono::NaiveDate;
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

    /// The desired delivery date of goods to consignee. The date must be a future date.
    /// If no date is provided then it will be calculated based on the SLA of the order type.
    /// Note that if you do not provide a deliveryDate you will have to provide the order type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_date: Option<NaiveDate>,

    /// Custom internal reference number of goods owner.
    pub reference_number: ReferenceNumber,

    /// The type of order decides such factors as SLA. Order types are decided between goods owner
    /// and us.
    ///
    /// NOTE: This is optional, even though the documentation states it isn't!
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,

    /// The designated recipient of the ordered goods.
    pub consignee: Consignee,

    /// The order lines decide which goods will be sent to the consignee. There can only be one
    /// article number per order line.
    pub order_lines: Vec<OrderLine>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    pub order_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_owner_id: Option<u32>,
    pub order_status: OrderStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_owner_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stockroom: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
