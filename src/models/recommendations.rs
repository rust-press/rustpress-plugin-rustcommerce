//! Product Recommendations Models
//!
//! Self-contained recommendation engine using collaborative filtering,
//! content-based filtering, and rule-based recommendations.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Product recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductRecommendation {
    pub product_id: Uuid,
    pub product_name: String,
    pub product_image: Option<String>,
    pub product_url: Option<String>,
    pub price: Decimal,
    pub sale_price: Option<Decimal>,
    pub recommendation_type: RecommendationType,
    pub score: Decimal,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationType {
    // Behavioral
    RecentlyViewed,
    FrequentlyBoughtTogether,
    CustomersAlsoBought,
    CustomersAlsoViewed,

    // Content-based
    SimilarProducts,
    SameCategory,
    SameBrand,
    SameAttributes,

    // Rule-based
    CrossSell,
    UpSell,
    RelatedProducts,

    // Personalized
    Personalized,
    ForYou,
    BasedOnHistory,

    // Trending
    BestSellers,
    Trending,
    NewArrivals,
    OnSale,

    // Cart-based
    CartComplement,
    CartUpgrade,
}

/// Recommendation widget placement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationWidget {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub title: String,
    pub placement: WidgetPlacement,
    pub recommendation_types: Vec<RecommendationType>,
    pub max_products: i32,
    pub layout: WidgetLayout,
    pub is_enabled: bool,

    // Filtering
    pub exclude_out_of_stock: bool,
    pub exclude_product_ids: Vec<Uuid>,
    pub include_category_ids: Option<Vec<Uuid>>,
    pub exclude_category_ids: Vec<Uuid>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,

    // Display settings
    pub show_price: bool,
    pub show_rating: bool,
    pub show_add_to_cart: bool,
    pub carousel_enabled: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WidgetPlacement {
    ProductPage,
    CartPage,
    CheckoutPage,
    HomePage,
    CategoryPage,
    SearchResults,
    OrderConfirmation,
    Email,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WidgetLayout {
    Grid,
    Carousel,
    List,
    Compact,
}

/// Product similarity data (for content-based filtering)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSimilarity {
    pub id: Uuid,
    pub product_id: Uuid,
    pub similar_product_id: Uuid,
    pub similarity_score: Decimal,
    pub similarity_type: SimilarityType,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SimilarityType {
    Category,
    Attributes,
    Description,
    Tags,
    Combined,
}

/// Co-purchase data (for collaborative filtering)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoPurchase {
    pub id: Uuid,
    pub product_id: Uuid,
    pub co_product_id: Uuid,
    pub purchase_count: i32,
    pub confidence: Decimal,
    pub lift: Decimal, // Statistical significance
    pub calculated_at: DateTime<Utc>,
}

/// Product view correlation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewCorrelation {
    pub id: Uuid,
    pub product_id: Uuid,
    pub correlated_product_id: Uuid,
    pub view_count: i32,
    pub correlation_score: Decimal,
    pub calculated_at: DateTime<Utc>,
}

/// Customer product affinity (personalization)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAffinity {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub entity_type: AffinityEntityType,
    pub entity_id: Uuid,
    pub affinity_score: Decimal,
    pub interaction_count: i32,
    pub last_interaction_at: DateTime<Utc>,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AffinityEntityType {
    Product,
    Category,
    Brand,
    Tag,
    Attribute,
}

/// Recommendation rule (manual cross-sell/up-sell)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationRule {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub rule_type: RuleType,
    pub is_enabled: bool,
    pub priority: i32,

    // Source products
    pub source_type: SourceType,
    pub source_product_ids: Option<Vec<Uuid>>,
    pub source_category_ids: Option<Vec<Uuid>>,

    // Target products to recommend
    pub target_product_ids: Vec<Uuid>,

    // Conditions
    pub min_cart_value: Option<Decimal>,
    pub max_cart_value: Option<Decimal>,
    pub customer_segments: Option<Vec<String>>,

    // Scheduling
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleType {
    CrossSell,
    UpSell,
    Related,
    Alternative,
    Accessory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    SpecificProducts,
    Category,
    AllProducts,
}

/// Trending product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingProduct {
    pub product_id: Uuid,
    pub trending_score: Decimal,
    pub views_24h: i64,
    pub views_7d: i64,
    pub sales_24h: i32,
    pub sales_7d: i32,
    pub growth_rate: Decimal,
    pub calculated_at: DateTime<Utc>,
}

/// Best seller
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestSeller {
    pub product_id: Uuid,
    pub category_id: Option<Uuid>,
    pub period: BestSellerPeriod,
    pub rank: i32,
    pub units_sold: i32,
    pub revenue: Decimal,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BestSellerPeriod {
    Daily,
    Weekly,
    Monthly,
    AllTime,
}

/// Recommendation interaction (tracking)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationInteraction {
    pub id: Uuid,
    pub widget_id: Uuid,
    pub recommendation_type: RecommendationType,
    pub product_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub interaction_type: InteractionType,
    pub context_product_id: Option<Uuid>, // Product page where shown
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionType {
    Impression,
    Click,
    AddToCart,
    Purchase,
}

/// Recommendation analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationAnalytics {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    // Overall
    pub total_impressions: i64,
    pub total_clicks: i64,
    pub total_conversions: i64,
    pub total_revenue: Decimal,

    // Rates
    pub click_through_rate: Decimal,
    pub conversion_rate: Decimal,
    pub revenue_per_impression: Decimal,

    // By type
    pub stats_by_type: Vec<TypeStats>,

    // By widget
    pub stats_by_widget: Vec<WidgetStats>,

    // Top performing products
    pub top_recommended_products: Vec<ProductStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeStats {
    pub recommendation_type: RecommendationType,
    pub impressions: i64,
    pub clicks: i64,
    pub conversions: i64,
    pub revenue: Decimal,
    pub ctr: Decimal,
    pub conversion_rate: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetStats {
    pub widget_id: Uuid,
    pub widget_name: String,
    pub impressions: i64,
    pub clicks: i64,
    pub conversions: i64,
    pub revenue: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductStats {
    pub product_id: Uuid,
    pub product_name: String,
    pub times_recommended: i64,
    pub clicks: i64,
    pub conversions: i64,
    pub revenue: Decimal,
}

/// Recommendation engine settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationSettings {
    pub enabled: bool,

    // Algorithm weights
    pub collaborative_weight: Decimal,
    pub content_based_weight: Decimal,
    pub rule_based_weight: Decimal,
    pub trending_weight: Decimal,

    // Collaborative filtering
    pub min_co_purchases: i32,
    pub min_confidence: Decimal,
    pub lookback_days: i32,

    // Content-based
    pub similarity_threshold: Decimal,
    pub max_similar_products: i32,

    // Trending
    pub trending_lookback_hours: i32,
    pub trending_sales_weight: Decimal,
    pub trending_views_weight: Decimal,

    // Personalization
    pub enable_personalization: bool,
    pub min_interactions: i32,
    pub affinity_decay_days: i32,

    // Cache
    pub cache_ttl_minutes: i32,
    pub recalculate_interval_hours: i32,
}

impl Default for RecommendationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            collaborative_weight: Decimal::new(40, 2), // 0.40
            content_based_weight: Decimal::new(30, 2), // 0.30
            rule_based_weight: Decimal::new(20, 2),    // 0.20
            trending_weight: Decimal::new(10, 2),      // 0.10
            min_co_purchases: 3,
            min_confidence: Decimal::new(1, 2), // 0.01
            lookback_days: 90,
            similarity_threshold: Decimal::new(50, 2), // 0.50
            max_similar_products: 20,
            trending_lookback_hours: 72,
            trending_sales_weight: Decimal::new(60, 2),
            trending_views_weight: Decimal::new(40, 2),
            enable_personalization: true,
            min_interactions: 5,
            affinity_decay_days: 30,
            cache_ttl_minutes: 60,
            recalculate_interval_hours: 24,
        }
    }
}

/// Get recommendations request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecommendationsRequest {
    pub placement: WidgetPlacement,
    pub context_product_id: Option<Uuid>,
    pub context_category_id: Option<Uuid>,
    pub cart_product_ids: Option<Vec<Uuid>>,
    pub customer_id: Option<Uuid>,
    pub limit: Option<i32>,
    pub exclude_product_ids: Option<Vec<Uuid>>,
    pub recommendation_types: Option<Vec<RecommendationType>>,
}

/// Get recommendations response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecommendationsResponse {
    pub recommendations: Vec<ProductRecommendation>,
    pub widget_id: Option<Uuid>,
    pub widget_title: Option<String>,
    pub total_available: i32,
}

/// Recommendation calculation job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationJob {
    pub id: Uuid,
    pub job_type: CalculationJobType,
    pub status: JobStatus,
    pub progress: i32,
    pub total_items: i32,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalculationJobType {
    Similarity,
    CoPurchase,
    Trending,
    BestSellers,
    CustomerAffinity,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl ProductRecommendation {
    /// Get effective price
    pub fn effective_price(&self) -> Decimal {
        self.sale_price.unwrap_or(self.price)
    }

    /// Check if on sale
    pub fn is_on_sale(&self) -> bool {
        self.sale_price.is_some() && self.sale_price < Some(self.price)
    }

    /// Get discount percentage
    pub fn discount_percentage(&self) -> Option<Decimal> {
        if let Some(sale) = self.sale_price {
            if sale < self.price {
                return Some((self.price - sale) / self.price * Decimal::from(100));
            }
        }
        None
    }
}
