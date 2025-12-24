//! Auction Models
//!
//! Product auction and bidding system.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Auction product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionProduct {
    pub id: Uuid,
    pub product_id: Uuid,
    pub auction_type: AuctionType,
    pub status: AuctionStatus,

    // Pricing
    pub starting_price: Decimal,
    pub reserve_price: Option<Decimal>,
    pub buy_now_price: Option<Decimal>,
    pub bid_increment: Decimal,
    pub min_bid_increment: Option<Decimal>,

    // Timing
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub extended_end_date: Option<DateTime<Utc>>,
    pub auto_extend: bool,
    pub extend_minutes: Option<i32>,
    pub extend_threshold_minutes: Option<i32>,

    // Current state
    pub current_bid: Option<Decimal>,
    pub bid_count: i32,
    pub highest_bidder_id: Option<Uuid>,
    pub watchers_count: i32,

    // Settings
    pub sealed_bids: bool, // Bids not visible until end
    pub proxy_bidding: bool,
    pub require_shipping: bool,
    pub allow_cancellation: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuctionType {
    Standard,      // Traditional ascending bid
    Dutch,         // Price decreases until someone bids
    Reverse,       // Lowest bid wins
    Penny,         // Each bid adds small amount
    Silent,        // Sealed bids revealed at end
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuctionStatus {
    Scheduled,
    Active,
    Extended,
    Ended,
    Sold,
    Unsold,       // No bids or reserve not met
    Cancelled,
    BuyNowPurchased,
}

/// Bid on an auction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub id: Uuid,
    pub auction_id: Uuid,
    pub customer_id: Uuid,
    pub bid_amount: Decimal,
    pub max_bid: Option<Decimal>, // For proxy bidding
    pub bid_type: BidType,
    pub status: BidStatus,
    pub is_winning: bool,
    pub outbid_notified: bool,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BidType {
    Manual,
    Proxy,      // Automatic bidding up to max
    BuyNow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BidStatus {
    Active,
    Outbid,
    Winning,
    Won,
    Lost,
    Retracted,
    Invalid,
}

/// Auction watcher (user watching an auction)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionWatcher {
    pub id: Uuid,
    pub auction_id: Uuid,
    pub customer_id: Uuid,
    pub email: Option<String>,
    pub notify_on_bid: bool,
    pub notify_on_ending: bool,
    pub notify_on_end: bool,
    pub created_at: DateTime<Utc>,
}

/// Auction settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionSettings {
    pub enabled: bool,
    pub default_duration_days: i32,
    pub max_duration_days: i32,
    pub auto_relist: bool,
    pub auto_relist_times: i32,
    pub bid_increment_type: BidIncrementType,
    pub default_bid_increment: Decimal,
    pub bid_increments: Vec<BidIncrementRange>,
    pub anti_sniping_enabled: bool,
    pub anti_sniping_minutes: i32,
    pub anti_sniping_extend_minutes: i32,
    pub require_payment_within_days: i32,
    pub allow_bid_retraction: bool,
    pub retraction_hours_before_end: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BidIncrementType {
    Fixed,
    Percentage,
    Tiered,
}

/// Bid increment range (for tiered increments)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidIncrementRange {
    pub min_price: Decimal,
    pub max_price: Option<Decimal>,
    pub increment: Decimal,
}

/// Dutch auction state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DutchAuctionState {
    pub auction_id: Uuid,
    pub current_price: Decimal,
    pub price_drop_amount: Decimal,
    pub price_drop_interval_minutes: i32,
    pub floor_price: Decimal,
    pub last_drop_at: DateTime<Utc>,
}

/// Auction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionResult {
    pub id: Uuid,
    pub auction_id: Uuid,
    pub product_id: Uuid,
    pub winning_bid_id: Option<Uuid>,
    pub winner_id: Option<Uuid>,
    pub final_price: Option<Decimal>,
    pub result_type: AuctionResultType,
    pub reserve_met: bool,
    pub order_id: Option<Uuid>,
    pub payment_status: PaymentStatus,
    pub payment_due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuctionResultType {
    Won,
    BuyNow,
    NoBids,
    ReserveNotMet,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Paid,
    Overdue,
    Refunded,
    Disputed,
}

/// Auction activity log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionActivity {
    pub id: Uuid,
    pub auction_id: Uuid,
    pub activity_type: ActivityType,
    pub customer_id: Option<Uuid>,
    pub bid_id: Option<Uuid>,
    pub amount: Option<Decimal>,
    pub description: String,
    pub meta: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    AuctionStarted,
    AuctionEnded,
    AuctionExtended,
    AuctionCancelled,
    BidPlaced,
    BidRetracted,
    ProxyBidActivated,
    ReserveMetNotice,
    BuyNowPurchased,
    WatchAdded,
    WatchRemoved,
}

impl AuctionProduct {
    /// Check if auction is currently active
    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        let end = self.extended_end_date.unwrap_or(self.end_date);
        matches!(self.status, AuctionStatus::Active | AuctionStatus::Extended)
            && now >= self.start_date
            && now < end
    }

    /// Check if auction has ended
    pub fn has_ended(&self) -> bool {
        let now = Utc::now();
        let end = self.extended_end_date.unwrap_or(self.end_date);
        now >= end || matches!(
            self.status,
            AuctionStatus::Ended | AuctionStatus::Sold | AuctionStatus::Unsold | AuctionStatus::Cancelled
        )
    }

    /// Get time remaining in seconds
    pub fn time_remaining(&self) -> Option<i64> {
        if self.is_active() {
            let end = self.extended_end_date.unwrap_or(self.end_date);
            Some((end - Utc::now()).num_seconds().max(0))
        } else {
            None
        }
    }

    /// Calculate minimum next bid
    pub fn min_next_bid(&self) -> Decimal {
        match self.current_bid {
            Some(current) => current + self.bid_increment,
            None => self.starting_price,
        }
    }

    /// Check if reserve price is met
    pub fn reserve_met(&self) -> bool {
        match (self.current_bid, self.reserve_price) {
            (Some(bid), Some(reserve)) => bid >= reserve,
            (Some(_), None) => true,
            _ => false,
        }
    }

    /// Check if buy now is available
    pub fn buy_now_available(&self) -> bool {
        self.buy_now_price.is_some()
            && self.is_active()
            && self.current_bid.map_or(true, |bid| {
                self.buy_now_price.map_or(false, |buy_now| bid < buy_now)
            })
    }

    /// Should extend auction (anti-sniping)
    pub fn should_extend(&self, threshold_minutes: i32) -> bool {
        if !self.auto_extend {
            return false;
        }
        let now = Utc::now();
        let end = self.extended_end_date.unwrap_or(self.end_date);
        let seconds_remaining = (end - now).num_seconds();
        seconds_remaining > 0 && seconds_remaining <= (threshold_minutes * 60) as i64
    }
}

impl Bid {
    /// Check if this bid is currently winning
    pub fn is_currently_winning(&self) -> bool {
        matches!(self.status, BidStatus::Active | BidStatus::Winning)
    }
}

impl Default for AuctionSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            default_duration_days: 7,
            max_duration_days: 30,
            auto_relist: false,
            auto_relist_times: 0,
            bid_increment_type: BidIncrementType::Tiered,
            default_bid_increment: Decimal::ONE,
            bid_increments: vec![
                BidIncrementRange {
                    min_price: Decimal::ZERO,
                    max_price: Some(Decimal::from(25)),
                    increment: Decimal::new(50, 2), // $0.50
                },
                BidIncrementRange {
                    min_price: Decimal::from(25),
                    max_price: Some(Decimal::from(100)),
                    increment: Decimal::ONE,
                },
                BidIncrementRange {
                    min_price: Decimal::from(100),
                    max_price: Some(Decimal::from(500)),
                    increment: Decimal::from(5),
                },
                BidIncrementRange {
                    min_price: Decimal::from(500),
                    max_price: None,
                    increment: Decimal::from(10),
                },
            ],
            anti_sniping_enabled: true,
            anti_sniping_minutes: 5,
            anti_sniping_extend_minutes: 5,
            require_payment_within_days: 3,
            allow_bid_retraction: true,
            retraction_hours_before_end: 12,
        }
    }
}

/// Place bid request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceBidRequest {
    pub auction_id: Uuid,
    pub amount: Decimal,
    pub max_bid: Option<Decimal>,
    pub is_proxy: bool,
}

/// Bid result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidResult {
    pub success: bool,
    pub bid_id: Option<Uuid>,
    pub is_winning: bool,
    pub current_price: Decimal,
    pub next_min_bid: Decimal,
    pub message: String,
    pub error: Option<BidError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BidError {
    AuctionNotActive,
    BidTooLow,
    BidBelowIncrement,
    MaxBidTooLow,
    CannotBidOnOwnAuction,
    BidderBlocked,
    AuctionEnded,
    InsufficientFunds,
}
