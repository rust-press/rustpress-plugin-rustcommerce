//! Advanced Search and Filtering Models
//!
//! Self-contained search system with faceted filtering and autocomplete.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: Option<String>,
    pub filters: Vec<SearchFilter>,
    pub sort: Option<SearchSort>,
    pub page: i32,
    pub per_page: i32,
    pub include_facets: bool,
    pub facet_attributes: Option<Vec<String>>,
    pub highlight: bool,
    pub fuzzy: bool,
    pub synonyms: bool,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            query: None,
            filters: Vec::new(),
            sort: None,
            page: 1,
            per_page: 24,
            include_facets: true,
            facet_attributes: None,
            highlight: true,
            fuzzy: true,
            synonyms: true,
        }
    }
}

/// Search filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilter {
    pub field: String,
    pub filter_type: FilterType,
    pub value: FilterValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterType {
    Equals,
    NotEquals,
    Contains,
    Range,
    In,
    NotIn,
    Exists,
    NotExists,
    Prefix,
    Suffix,
    Regex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilterValue {
    Single(String),
    Multiple(Vec<String>),
    Range { min: Option<Decimal>, max: Option<Decimal> },
    Boolean(bool),
}

/// Search sort
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSort {
    pub field: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Asc,
    Desc,
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub query: String,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
    pub hits: Vec<SearchHit>,
    pub facets: Vec<SearchFacet>,
    pub suggestions: Vec<String>,
    pub did_you_mean: Option<String>,
    pub search_time_ms: i64,
}

/// Search hit (individual result)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub sku: Option<String>,
    pub price: Decimal,
    pub sale_price: Option<Decimal>,
    pub image_url: Option<String>,
    pub category_id: Option<Uuid>,
    pub category_name: Option<String>,
    pub brand: Option<String>,
    pub rating: Option<Decimal>,
    pub review_count: i32,
    pub stock_status: String,
    pub attributes: HashMap<String, String>,
    pub highlights: Option<SearchHighlights>,
    pub score: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHighlights {
    pub name: Option<String>,
    pub description: Option<String>,
    pub sku: Option<String>,
}

/// Search facet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFacet {
    pub name: String,
    pub field: String,
    pub facet_type: FacetType,
    pub values: Vec<FacetValue>,
    pub display_order: i32,
    pub collapsed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FacetType {
    Terms,
    Range,
    DateRange,
    Hierarchical,
    Toggle,
    Rating,
    Color,
    Size,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacetValue {
    pub value: String,
    pub label: String,
    pub count: i64,
    pub selected: bool,
    pub color_code: Option<String>,
    pub range_min: Option<Decimal>,
    pub range_max: Option<Decimal>,
    pub children: Option<Vec<FacetValue>>,
}

/// Autocomplete suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteSuggestion {
    pub suggestion_type: SuggestionType,
    pub text: String,
    pub highlighted_text: String,
    pub product_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub image_url: Option<String>,
    pub price: Option<Decimal>,
    pub score: Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionType {
    Product,
    Category,
    Brand,
    Keyword,
    Recent,
    Popular,
}

/// Search synonym
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSynonym {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub synonym_type: SynonymType,
    pub terms: Vec<String>,
    pub replacement: Option<String>,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SynonymType {
    Equivalent,  // All terms are equal (tv = television)
    OneWay,      // A -> B (cheap -> affordable)
    Expansion,   // A -> A + B (phone -> phone + smartphone)
}

/// Search redirect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRedirect {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub search_term: String,
    pub match_type: MatchType,
    pub redirect_url: String,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchType {
    Exact,
    Contains,
    StartsWith,
    Regex,
}

/// Search merchandising rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchandisingRule {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub rule_type: MerchandisingRuleType,
    pub search_terms: Vec<String>,
    pub match_type: MatchType,
    pub action: MerchandisingAction,
    pub priority: i32,
    pub is_enabled: bool,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MerchandisingRuleType {
    Pin,
    Boost,
    Bury,
    Filter,
    Banner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchandisingAction {
    pub action_type: MerchandisingRuleType,
    pub product_ids: Option<Vec<Uuid>>,
    pub boost_factor: Option<Decimal>,
    pub filter: Option<SearchFilter>,
    pub banner_content: Option<String>,
    pub banner_image: Option<String>,
    pub banner_link: Option<String>,
}

/// Search history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistory {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub query: String,
    pub filters_used: Vec<String>,
    pub result_count: i64,
    pub clicked_products: Vec<Uuid>,
    pub converted: bool,
    pub order_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Popular search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopularSearch {
    pub query: String,
    pub search_count: i64,
    pub click_count: i64,
    pub conversion_count: i64,
    pub conversion_rate: Decimal,
    pub last_searched_at: DateTime<Utc>,
}

/// Search analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAnalytics {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    // Volume
    pub total_searches: i64,
    pub unique_searches: i64,
    pub searches_with_results: i64,
    pub searches_without_results: i64,

    // Performance
    pub avg_results_per_search: Decimal,
    pub avg_search_time_ms: Decimal,
    pub zero_result_rate: Decimal,

    // Engagement
    pub click_through_rate: Decimal,
    pub avg_clicks_per_search: Decimal,
    pub search_to_cart_rate: Decimal,
    pub search_to_purchase_rate: Decimal,

    // Revenue
    pub revenue_from_search: Decimal,
    pub avg_order_value_search: Decimal,

    // Top data
    pub top_searches: Vec<PopularSearch>,
    pub top_zero_result_searches: Vec<ZeroResultSearch>,
    pub top_converting_searches: Vec<ConvertingSearch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroResultSearch {
    pub query: String,
    pub search_count: i64,
    pub suggested_action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertingSearch {
    pub query: String,
    pub search_count: i64,
    pub conversion_count: i64,
    pub revenue: Decimal,
    pub conversion_rate: Decimal,
}

/// Search index configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndexConfig {
    pub searchable_fields: Vec<SearchableField>,
    pub filterable_fields: Vec<String>,
    pub sortable_fields: Vec<String>,
    pub facet_fields: Vec<FacetConfig>,
    pub ranking_rules: Vec<RankingRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchableField {
    pub field: String,
    pub weight: Decimal,
    pub prefix_search: bool,
    pub fuzzy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacetConfig {
    pub field: String,
    pub display_name: String,
    pub facet_type: FacetType,
    pub display_order: i32,
    pub collapsed: bool,
    pub max_values: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingRule {
    pub rule_type: RankingRuleType,
    pub field: Option<String>,
    pub direction: Option<SortDirection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankingRuleType {
    Words,
    Typo,
    Proximity,
    Attribute,
    Sort,
    Exactness,
    Custom,
}

/// Search settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSettings {
    pub enabled: bool,
    pub min_query_length: i32,
    pub max_query_length: i32,
    pub enable_autocomplete: bool,
    pub autocomplete_min_chars: i32,
    pub autocomplete_max_suggestions: i32,
    pub enable_fuzzy: bool,
    pub fuzzy_max_edits: i32,
    pub enable_synonyms: bool,
    pub enable_popular_searches: bool,
    pub enable_recent_searches: bool,
    pub enable_redirects: bool,
    pub enable_merchandising: bool,
    pub highlight_tag: String,
    pub results_per_page: i32,
    pub max_results_per_page: i32,
}

impl Default for SearchSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            min_query_length: 2,
            max_query_length: 100,
            enable_autocomplete: true,
            autocomplete_min_chars: 2,
            autocomplete_max_suggestions: 8,
            enable_fuzzy: true,
            fuzzy_max_edits: 2,
            enable_synonyms: true,
            enable_popular_searches: true,
            enable_recent_searches: true,
            enable_redirects: true,
            enable_merchandising: true,
            highlight_tag: "em".to_string(),
            results_per_page: 24,
            max_results_per_page: 100,
        }
    }
}

/// Autocomplete request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteRequest {
    pub query: String,
    pub limit: Option<i32>,
    pub include_products: Option<bool>,
    pub include_categories: Option<bool>,
    pub include_brands: Option<bool>,
    pub include_recent: Option<bool>,
    pub include_popular: Option<bool>,
}

/// Autocomplete response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteResponse {
    pub suggestions: Vec<AutocompleteSuggestion>,
    pub products: Vec<SearchHit>,
    pub categories: Vec<CategorySuggestion>,
    pub brands: Vec<BrandSuggestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySuggestion {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub product_count: i64,
    pub highlighted_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandSuggestion {
    pub name: String,
    pub slug: String,
    pub product_count: i64,
    pub logo_url: Option<String>,
    pub highlighted_name: String,
}
