//! Wishlist Models
//!
//! Customer wishlists and favorites functionality.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Wishlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wishlist {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub session_id: Option<String>, // For guest wishlists
    pub name: String,
    pub slug: String,
    pub privacy: WishlistPrivacy,
    pub is_default: bool,
    pub share_key: Option<String>,
    pub item_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WishlistPrivacy {
    Private,
    Shared,  // Anyone with link
    Public,  // Visible in search
}

/// Wishlist item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WishlistItem {
    pub id: Uuid,
    pub wishlist_id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub quantity: i32,

    // Product snapshot (for display even if product changes)
    pub product_name: String,
    pub product_price: Decimal,
    pub product_image: Option<String>,

    // Variation attributes
    pub variation_attributes: HashMap<String, String>,

    // User notes
    pub note: Option<String>,
    pub priority: WishlistPriority,

    // Price tracking
    pub price_when_added: Decimal,
    pub on_sale_when_added: bool,

    // Dates
    pub added_at: DateTime<Utc>,
    pub purchased_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WishlistPriority {
    Low,
    Medium,
    High,
}

/// Wishlist share
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WishlistShare {
    pub id: Uuid,
    pub wishlist_id: Uuid,
    pub share_type: ShareType,
    pub recipient_email: Option<String>,
    pub share_key: String,
    pub message: Option<String>,
    pub views: i32,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShareType {
    Link,
    Email,
    Social,
}

/// Back in stock notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockNotification {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub email: String,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub status: NotificationStatus,
    pub notified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationStatus {
    Active,
    Notified,
    Purchased,
    Unsubscribed,
}

/// Price drop notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceDropNotification {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub email: String,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub target_price: Option<Decimal>,
    pub original_price: Decimal,
    pub status: NotificationStatus,
    pub notified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Wishlist analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WishlistAnalytics {
    pub product_id: Uuid,
    pub wishlist_count: i64,
    pub purchased_from_wishlist: i64,
    pub conversion_rate: Decimal,
    pub avg_days_to_purchase: Option<f64>,
}

impl Wishlist {
    /// Generate share URL
    pub fn get_share_url(&self, base_url: &str) -> Option<String> {
        self.share_key.as_ref().map(|key| {
            format!("{}/wishlist/{}/{}", base_url, self.slug, key)
        })
    }

    /// Generate new share key
    pub fn generate_share_key() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..16)
            .map(|_| {
                let idx = rng.gen_range(0..36);
                if idx < 10 {
                    (b'0' + idx) as char
                } else {
                    (b'a' + idx - 10) as char
                }
            })
            .collect()
    }

    /// Check if accessible by user
    pub fn is_accessible_by(&self, customer_id: Option<Uuid>, share_key: Option<&str>) -> bool {
        match self.privacy {
            WishlistPrivacy::Private => self.customer_id == customer_id,
            WishlistPrivacy::Shared => {
                self.customer_id == customer_id ||
                share_key.map_or(false, |k| self.share_key.as_deref() == Some(k))
            }
            WishlistPrivacy::Public => true,
        }
    }
}

impl WishlistItem {
    /// Check if price dropped since added
    pub fn price_dropped(&self) -> bool {
        self.product_price < self.price_when_added
    }

    /// Get price drop percentage
    pub fn price_drop_percentage(&self) -> Option<Decimal> {
        if self.product_price < self.price_when_added {
            let drop = self.price_when_added - self.product_price;
            Some((drop / self.price_when_added) * Decimal::from(100))
        } else {
            None
        }
    }
}

/// Wishlist request DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWishlistRequest {
    pub name: String,
    pub privacy: Option<WishlistPrivacy>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddToWishlistRequest {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub quantity: Option<i32>,
    pub note: Option<String>,
    pub priority: Option<WishlistPriority>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareWishlistRequest {
    pub share_type: ShareType,
    pub recipient_email: Option<String>,
    pub message: Option<String>,
    pub expires_days: Option<i32>,
}
