//! Shipping Service
//!
//! Handles shipping rate calculations, shipping zones, and shipping methods.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;
use std::collections::HashMap;

use crate::models::shipping::{
    ShippingZone, ShippingZoneLocation, ShippingZoneMethod, ShippingMethodSettings,
    ShippingClass, CalculatedShippingRate, ShippingPackage, ShippingDestination,
    LocationType, ShippingCalcType, ShippingTaxStatus,
};
use crate::models::cart::Cart;
use crate::settings::RustCommerceSettings;

/// Shipping service
pub struct ShippingService {
    settings: RustCommerceSettings,
}

/// Shipping calculation result
#[derive(Debug, Clone)]
pub struct ShippingCalculationResult {
    pub rates: Vec<CalculatedShippingRate>,
    pub packages: Vec<ShippingPackage>,
    pub needs_shipping: bool,
}

/// Shipping error
#[derive(Debug, Clone)]
pub enum ShippingError {
    NoShippingZone,
    NoShippingMethods,
    InvalidDestination,
    CalculationFailed(String),
}

impl std::fmt::Display for ShippingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoShippingZone => write!(f, "No shipping zone found for this destination"),
            Self::NoShippingMethods => write!(f, "No shipping methods available"),
            Self::InvalidDestination => write!(f, "Invalid shipping destination"),
            Self::CalculationFailed(msg) => write!(f, "Shipping calculation failed: {}", msg),
        }
    }
}

impl ShippingService {
    /// Create a new shipping service
    pub fn new(settings: RustCommerceSettings) -> Self {
        Self { settings }
    }

    /// Find matching shipping zone for destination
    pub fn find_zone(
        &self,
        destination: &ShippingDestination,
        zones: &[ShippingZone],
    ) -> Option<&ShippingZone> {
        // Sort zones by specificity (most specific first)
        let mut sorted_zones: Vec<&ShippingZone> = zones.iter().collect();
        sorted_zones.sort_by(|a, b| {
            // Higher specificity score = more specific
            let a_score = self.calculate_zone_specificity(a, destination);
            let b_score = self.calculate_zone_specificity(b, destination);
            b_score.cmp(&a_score)
        });

        // Find first matching zone
        for zone in sorted_zones {
            if self.zone_matches(zone, destination) {
                return Some(zone);
            }
        }

        // Return "rest of world" zone if exists (zone with no locations)
        zones.iter().find(|z| z.locations.is_empty())
    }

    /// Check if zone matches destination
    fn zone_matches(&self, zone: &ShippingZone, destination: &ShippingDestination) -> bool {
        if zone.locations.is_empty() {
            // Empty location list = "rest of world"
            return true;
        }

        for location in &zone.locations {
            match location.location_type {
                LocationType::Country => {
                    if location.location_code == destination.country {
                        return true;
                    }
                }
                LocationType::State => {
                    // Format: "US:CA" for California, US
                    let parts: Vec<&str> = location.location_code.split(':').collect();
                    if parts.len() == 2 {
                        if parts[0] == destination.country && parts[1] == destination.state {
                            return true;
                        }
                    }
                }
                LocationType::Postcode => {
                    if self.postcode_matches(&location.location_code, &destination.postcode) {
                        return true;
                    }
                }
                LocationType::Continent => {
                    if self.country_in_continent(&destination.country, &location.location_code) {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Calculate zone specificity score
    fn calculate_zone_specificity(&self, zone: &ShippingZone, destination: &ShippingDestination) -> i32 {
        let mut score = 0;

        for location in &zone.locations {
            match location.location_type {
                LocationType::Postcode => score = score.max(4),
                LocationType::State => score = score.max(3),
                LocationType::Country => score = score.max(2),
                LocationType::Continent => score = score.max(1),
            }
        }

        score
    }

    /// Check if postcode matches (supports wildcards and ranges)
    fn postcode_matches(&self, pattern: &str, postcode: &str) -> bool {
        if pattern.contains('*') {
            // Wildcard match
            let prefix = pattern.replace('*', "");
            postcode.starts_with(&prefix)
        } else if pattern.contains("...") {
            // Range match
            let parts: Vec<&str> = pattern.split("...").collect();
            if parts.len() == 2 {
                if let (Ok(start), Ok(end), Ok(pc)) = (
                    parts[0].parse::<i64>(),
                    parts[1].parse::<i64>(),
                    postcode.replace('-', "").replace(' ', "").parse::<i64>()
                ) {
                    return pc >= start && pc <= end;
                }
            }
            false
        } else {
            pattern == postcode
        }
    }

    /// Check if country is in continent
    fn country_in_continent(&self, country: &str, continent: &str) -> bool {
        let continents: HashMap<&str, Vec<&str>> = HashMap::from([
            ("NA", vec!["US", "CA", "MX"]),
            ("EU", vec!["GB", "DE", "FR", "IT", "ES", "NL", "BE", "AT", "CH", "PL", "SE", "NO", "DK", "FI"]),
            ("AS", vec!["CN", "JP", "KR", "IN", "SG", "HK", "TW", "TH", "MY", "ID", "PH", "VN"]),
            ("OC", vec!["AU", "NZ"]),
            ("SA", vec!["BR", "AR", "CL", "CO", "PE", "EC"]),
            ("AF", vec!["ZA", "EG", "NG", "KE", "MA"]),
        ]);

        continents.get(continent)
            .map_or(false, |countries| countries.contains(&country))
    }

    /// Calculate shipping rates for cart
    pub fn calculate_rates(
        &self,
        cart: &Cart,
        destination: &ShippingDestination,
        zones: &[ShippingZone],
        shipping_classes: &HashMap<Uuid, ShippingClass>,
    ) -> Result<ShippingCalculationResult, ShippingError> {
        // Check if cart needs shipping
        let needs_shipping = self.cart_needs_shipping(cart);
        if !needs_shipping {
            return Ok(ShippingCalculationResult {
                rates: vec![],
                packages: vec![],
                needs_shipping: false,
            });
        }

        // Find matching zone
        let zone = self.find_zone(destination, zones)
            .ok_or(ShippingError::NoShippingZone)?;

        // Get enabled methods
        let enabled_methods: Vec<&ShippingZoneMethod> = zone.methods.iter()
            .filter(|m| m.is_enabled)
            .collect();

        if enabled_methods.is_empty() {
            return Err(ShippingError::NoShippingMethods);
        }

        // Create packages
        let packages = self.create_packages(cart, destination);

        // Calculate rates for each method
        let mut rates = Vec::new();
        for method in enabled_methods {
            if let Some(rate) = self.calculate_method_rate(method, cart, &packages, shipping_classes) {
                rates.push(rate);
            }
        }

        // Sort by cost
        rates.sort_by(|a, b| a.cost.cmp(&b.cost));

        Ok(ShippingCalculationResult {
            rates,
            packages,
            needs_shipping,
        })
    }

    /// Calculate rate for a specific shipping method
    fn calculate_method_rate(
        &self,
        method: &ShippingZoneMethod,
        cart: &Cart,
        packages: &[ShippingPackage],
        shipping_classes: &HashMap<Uuid, ShippingClass>,
    ) -> Option<CalculatedShippingRate> {
        let settings = &method.settings;

        match method.method_id.as_str() {
            "flat_rate" => self.calculate_flat_rate(method, cart, packages, shipping_classes),
            "free_shipping" => self.calculate_free_shipping(method, cart),
            "local_pickup" => self.calculate_local_pickup(method),
            _ => None,
        }
    }

    /// Calculate flat rate shipping
    fn calculate_flat_rate(
        &self,
        method: &ShippingZoneMethod,
        cart: &Cart,
        packages: &[ShippingPackage],
        shipping_classes: &HashMap<Uuid, ShippingClass>,
    ) -> Option<CalculatedShippingRate> {
        let settings = &method.settings;
        let mut cost = settings.cost.unwrap_or(Decimal::ZERO);

        // Add per-item cost
        if let Some(per_item) = settings.cost_per_item {
            let item_count: i32 = cart.items.iter().map(|i| i.quantity).sum();
            cost += per_item * Decimal::from(item_count);
        }

        // Add weight-based cost
        if let Some(per_weight) = settings.cost_per_weight_unit {
            let total_weight: Decimal = packages.iter()
                .map(|p| p.contents_weight)
                .sum();
            cost += per_weight * total_weight;
        }

        // Add shipping class costs
        let calc_type = settings.calc_type.unwrap_or(ShippingCalcType::PerOrder);
        cost += self.calculate_class_costs(cart, settings, &calc_type);

        Some(CalculatedShippingRate {
            id: format!("flat_rate:{}", method.id),
            method_id: "flat_rate".to_string(),
            instance_id: method.id.to_string(),
            label: "Flat rate".to_string(),
            cost,
            taxes: HashMap::new(),
            meta: HashMap::new(),
            package_id: packages.first().map(|p| p.id.clone()).unwrap_or_default(),
        })
    }

    /// Calculate shipping class costs
    fn calculate_class_costs(
        &self,
        cart: &Cart,
        settings: &ShippingMethodSettings,
        calc_type: &ShippingCalcType,
    ) -> Decimal {
        // Simplified - in full implementation would check each item's shipping class
        Decimal::ZERO
    }

    /// Calculate free shipping
    fn calculate_free_shipping(
        &self,
        method: &ShippingZoneMethod,
        cart: &Cart,
    ) -> Option<CalculatedShippingRate> {
        let settings = &method.settings;

        // Check minimum amount
        if let Some(min_amount) = settings.min_amount {
            if cart.totals.subtotal < min_amount {
                return None;
            }
        }

        // Check if requires coupon
        if settings.requires_coupon.unwrap_or(false) {
            let has_free_shipping_coupon = cart.coupons.iter()
                .any(|c| c.free_shipping);
            if !has_free_shipping_coupon {
                return None;
            }
        }

        Some(CalculatedShippingRate {
            id: format!("free_shipping:{}", method.id),
            method_id: "free_shipping".to_string(),
            instance_id: method.id.to_string(),
            label: "Free shipping".to_string(),
            cost: Decimal::ZERO,
            taxes: HashMap::new(),
            meta: HashMap::new(),
            package_id: String::new(),
        })
    }

    /// Calculate local pickup
    fn calculate_local_pickup(
        &self,
        method: &ShippingZoneMethod,
    ) -> Option<CalculatedShippingRate> {
        let settings = &method.settings;

        let mut meta = HashMap::new();
        if let Some(ref location) = settings.pickup_location {
            meta.insert("pickup_location".to_string(), location.clone());
        }

        Some(CalculatedShippingRate {
            id: format!("local_pickup:{}", method.id),
            method_id: "local_pickup".to_string(),
            instance_id: method.id.to_string(),
            label: "Local pickup".to_string(),
            cost: settings.cost.unwrap_or(Decimal::ZERO),
            taxes: HashMap::new(),
            meta,
            package_id: String::new(),
        })
    }

    /// Create shipping packages from cart
    fn create_packages(&self, cart: &Cart, destination: &ShippingDestination) -> Vec<ShippingPackage> {
        // For simplicity, create single package
        // In full implementation, could split by shipping class, vendor, etc.
        vec![ShippingPackage {
            id: "package_0".to_string(),
            contents_cost: cart.totals.subtotal,
            contents_weight: Decimal::ZERO, // Would calculate from items
            destination: destination.clone(),
            items: cart.items.iter().map(|item| {
                crate::models::shipping::PackageItem {
                    product_id: item.product_id,
                    variation_id: item.variation_id,
                    quantity: item.quantity,
                    weight: None,
                    shipping_class_id: None,
                }
            }).collect(),
        }]
    }

    /// Check if cart needs shipping
    fn cart_needs_shipping(&self, cart: &Cart) -> bool {
        // In full implementation, check if any items are non-virtual
        !cart.items.is_empty()
    }

    /// Get available shipping methods for display
    pub fn get_available_methods(&self, zone: &ShippingZone) -> Vec<ShippingMethodInfo> {
        zone.methods.iter()
            .filter(|m| m.is_enabled)
            .map(|m| ShippingMethodInfo {
                id: m.id,
                method_id: m.method_id.clone(),
                title: self.get_method_title(&m.method_id),
                description: self.get_method_description(&m.method_id),
            })
            .collect()
    }

    /// Get method title
    fn get_method_title(&self, method_id: &str) -> String {
        match method_id {
            "flat_rate" => "Flat rate".to_string(),
            "free_shipping" => "Free shipping".to_string(),
            "local_pickup" => "Local pickup".to_string(),
            _ => method_id.to_string(),
        }
    }

    /// Get method description
    fn get_method_description(&self, method_id: &str) -> String {
        match method_id {
            "flat_rate" => "Fixed rate shipping".to_string(),
            "free_shipping" => "Free shipping for qualifying orders".to_string(),
            "local_pickup" => "Pick up from store location".to_string(),
            _ => String::new(),
        }
    }
}

/// Shipping method info
#[derive(Debug, Clone)]
pub struct ShippingMethodInfo {
    pub id: Uuid,
    pub method_id: String,
    pub title: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postcode_wildcard() {
        let settings = RustCommerceSettings::default();
        let service = ShippingService::new(settings);

        assert!(service.postcode_matches("9*", "90210"));
        assert!(service.postcode_matches("902*", "90210"));
        assert!(!service.postcode_matches("8*", "90210"));
    }

    #[test]
    fn test_postcode_range() {
        let settings = RustCommerceSettings::default();
        let service = ShippingService::new(settings);

        assert!(service.postcode_matches("90000...91000", "90210"));
        assert!(!service.postcode_matches("90000...90100", "90210"));
    }

    #[test]
    fn test_continent_lookup() {
        let settings = RustCommerceSettings::default();
        let service = ShippingService::new(settings);

        assert!(service.country_in_continent("US", "NA"));
        assert!(service.country_in_continent("GB", "EU"));
        assert!(!service.country_in_continent("US", "EU"));
    }
}
