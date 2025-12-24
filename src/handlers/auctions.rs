//! Auctions API Handlers
//!
//! HTTP request handlers for auction functionality.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AuctionQuery {
    pub status: Option<String>,
    pub ending_soon: Option<bool>,
    pub category_id: Option<Uuid>,
    pub min_price: Option<String>,
    pub max_price: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct PlaceBidRequest {
    pub auction_id: Uuid,
    pub amount: String,
    pub max_bid: Option<String>, // For auto-bidding
}

#[derive(Debug, Deserialize)]
pub struct WatchAuctionRequest {
    pub auction_id: Uuid,
    pub notify_outbid: bool,
    pub notify_ending: bool,
}

#[derive(Debug, Serialize)]
pub struct AuctionResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub product_image: Option<String>,
    pub status: String,
    pub start_price: String,
    pub current_bid: Option<String>,
    pub buy_now_price: Option<String>,
    pub bid_count: i32,
    pub start_date: String,
    pub end_date: String,
    pub time_remaining: Option<i64>,
    pub is_reserve_met: bool,
    pub highest_bidder_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct BidResponse {
    pub id: Uuid,
    pub auction_id: Uuid,
    pub bidder_id: Uuid,
    pub amount: String,
    pub is_winning: bool,
    pub bid_time: String,
}

#[derive(Debug, Serialize)]
pub struct BidHistoryResponse {
    pub bids: Vec<BidEntry>,
    pub total_bids: i32,
}

#[derive(Debug, Serialize)]
pub struct BidEntry {
    pub bidder_name: String,
    pub amount: String,
    pub bid_time: String,
    pub is_current_user: bool,
}

#[derive(Debug, Serialize)]
pub struct AuctionEndedResponse {
    pub auction_id: Uuid,
    pub winner_id: Option<Uuid>,
    pub winning_bid: Option<String>,
    pub reserve_met: bool,
    pub checkout_url: Option<String>,
}
