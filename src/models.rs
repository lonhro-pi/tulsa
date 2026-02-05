use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Subscriber {
    pub id: String,
    pub name: String,
    pub username: String,
    pub platform: String,
    pub tier: String, // "VIP", "Regular", "Casual"
    pub total_spent: f64,
    pub last_purchase_date: Option<DateTime<Utc>>,
    pub subscription_end_date: Option<DateTime<Utc>>,
    pub last_interaction_date: Option<DateTime<Utc>>,
    pub preferences: Option<String>, // JSON string for flexible data
    pub notes: Option<String>,
    pub birthday: Option<String>,
    pub favorite_content_types: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubscriber {
    pub name: String,
    pub username: String,
    pub platform: String,
    pub tier: Option<String>,
    pub birthday: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSubscriber {
    pub name: Option<String>,
    pub username: Option<String>,
    pub platform: Option<String>,
    pub tier: Option<String>,
    pub total_spent: Option<f64>,
    pub last_purchase_date: Option<DateTime<Utc>>,
    pub subscription_end_date: Option<DateTime<Utc>>,
    pub last_interaction_date: Option<DateTime<Utc>>,
    pub preferences: Option<String>,
    pub notes: Option<String>,
    pub birthday: Option<String>,
    pub favorite_content_types: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Interaction {
    pub id: String,
    pub subscriber_id: String,
    pub interaction_type: String, // "message", "purchase", "tip", "renewal"
    pub amount: Option<f64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInteraction {
    pub subscriber_id: String,
    pub interaction_type: String,
    pub amount: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberStats {
    pub total_subscribers: i64,
    pub vip_count: i64,
    pub regular_count: i64,
    pub casual_count: i64,
    pub total_revenue: f64,
    pub expiring_soon: i64, // Subscriptions expiring in next 7 days
}
