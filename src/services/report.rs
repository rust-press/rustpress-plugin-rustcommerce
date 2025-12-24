//! Report Service
//!
//! Handles sales reports, analytics, and dashboard statistics.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration, Datelike};
use std::collections::HashMap;

use crate::models::order::{Order, OrderStatus};
use crate::models::product::Product;
use crate::models::customer::Customer;
use crate::settings::RustCommerceSettings;

/// Report service
pub struct ReportService {
    settings: RustCommerceSettings,
}

/// Date range for reports
#[derive(Debug, Clone, Copy)]
pub enum DateRange {
    Today,
    Yesterday,
    ThisWeek,
    LastWeek,
    ThisMonth,
    LastMonth,
    ThisYear,
    LastYear,
    Custom(DateTime<Utc>, DateTime<Utc>),
}

/// Sales report
#[derive(Debug, Clone)]
pub struct SalesReport {
    pub period: String,
    pub total_sales: Decimal,
    pub net_sales: Decimal,
    pub total_orders: i32,
    pub total_items: i32,
    pub total_shipping: Decimal,
    pub total_tax: Decimal,
    pub total_refunds: Decimal,
    pub total_discount: Decimal,
    pub average_order_value: Decimal,
    pub orders_by_status: HashMap<OrderStatus, i32>,
}

/// Dashboard stats
#[derive(Debug, Clone)]
pub struct DashboardStats {
    pub net_sales: Decimal,
    pub orders_count: i32,
    pub items_sold: i32,
    pub refunded: Decimal,
    pub shipping: Decimal,
    pub average_order_value: Decimal,
    pub net_sales_change: Decimal, // Percentage change from previous period
    pub orders_change: Decimal,
}

/// Top seller
#[derive(Debug, Clone)]
pub struct TopSeller {
    pub product_id: Uuid,
    pub product_name: String,
    pub quantity_sold: i32,
    pub total_revenue: Decimal,
}

/// Sales by category
#[derive(Debug, Clone)]
pub struct CategorySales {
    pub category_id: Uuid,
    pub category_name: String,
    pub items_sold: i32,
    pub total_sales: Decimal,
}

/// Sales by date
#[derive(Debug, Clone)]
pub struct SalesByDate {
    pub date: String,
    pub orders: i32,
    pub items: i32,
    pub gross_sales: Decimal,
    pub net_sales: Decimal,
    pub shipping: Decimal,
    pub tax: Decimal,
    pub refunds: Decimal,
}

impl ReportService {
    /// Create a new report service
    pub fn new(settings: RustCommerceSettings) -> Self {
        Self { settings }
    }

    /// Get date range boundaries
    pub fn get_date_range(&self, range: DateRange) -> (DateTime<Utc>, DateTime<Utc>) {
        let now = Utc::now();

        match range {
            DateRange::Today => {
                let start = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
                (DateTime::from_naive_utc_and_offset(start, Utc), now)
            }
            DateRange::Yesterday => {
                let yesterday = now - Duration::days(1);
                let start = yesterday.date_naive().and_hms_opt(0, 0, 0).unwrap();
                let end = yesterday.date_naive().and_hms_opt(23, 59, 59).unwrap();
                (
                    DateTime::from_naive_utc_and_offset(start, Utc),
                    DateTime::from_naive_utc_and_offset(end, Utc)
                )
            }
            DateRange::ThisWeek => {
                let days_from_monday = now.weekday().num_days_from_monday() as i64;
                let monday = now - Duration::days(days_from_monday);
                let start = monday.date_naive().and_hms_opt(0, 0, 0).unwrap();
                (DateTime::from_naive_utc_and_offset(start, Utc), now)
            }
            DateRange::LastWeek => {
                let days_from_monday = now.weekday().num_days_from_monday() as i64;
                let this_monday = now - Duration::days(days_from_monday);
                let last_monday = this_monday - Duration::days(7);
                let last_sunday = this_monday - Duration::days(1);
                let start = last_monday.date_naive().and_hms_opt(0, 0, 0).unwrap();
                let end = last_sunday.date_naive().and_hms_opt(23, 59, 59).unwrap();
                (
                    DateTime::from_naive_utc_and_offset(start, Utc),
                    DateTime::from_naive_utc_and_offset(end, Utc)
                )
            }
            DateRange::ThisMonth => {
                let start = now.date_naive().with_day(1).unwrap().and_hms_opt(0, 0, 0).unwrap();
                (DateTime::from_naive_utc_and_offset(start, Utc), now)
            }
            DateRange::LastMonth => {
                let first_of_month = now.date_naive().with_day(1).unwrap();
                let last_month = first_of_month - Duration::days(1);
                let start = last_month.with_day(1).unwrap().and_hms_opt(0, 0, 0).unwrap();
                let end = last_month.and_hms_opt(23, 59, 59).unwrap();
                (
                    DateTime::from_naive_utc_and_offset(start, Utc),
                    DateTime::from_naive_utc_and_offset(end, Utc)
                )
            }
            DateRange::ThisYear => {
                let start = now.date_naive().with_month(1).unwrap().with_day(1).unwrap()
                    .and_hms_opt(0, 0, 0).unwrap();
                (DateTime::from_naive_utc_and_offset(start, Utc), now)
            }
            DateRange::LastYear => {
                let this_jan = now.date_naive().with_month(1).unwrap().with_day(1).unwrap();
                let last_jan = this_jan.with_year(this_jan.year() - 1).unwrap();
                let last_dec = this_jan - Duration::days(1);
                let start = last_jan.and_hms_opt(0, 0, 0).unwrap();
                let end = last_dec.and_hms_opt(23, 59, 59).unwrap();
                (
                    DateTime::from_naive_utc_and_offset(start, Utc),
                    DateTime::from_naive_utc_and_offset(end, Utc)
                )
            }
            DateRange::Custom(start, end) => (start, end),
        }
    }

    /// Filter orders by date range
    fn filter_orders_by_date<'a>(
        &self,
        orders: &'a [Order],
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&'a Order> {
        orders.iter()
            .filter(|o| o.created_at >= start && o.created_at <= end)
            .collect()
    }

    /// Get paid orders only
    fn get_paid_orders<'a>(&self, orders: &'a [Order]) -> Vec<&'a Order> {
        orders.iter()
            .filter(|o| {
                matches!(
                    o.status,
                    OrderStatus::Processing | OrderStatus::Completed | OrderStatus::OnHold
                )
            })
            .collect()
    }

    /// Generate sales report
    pub fn generate_sales_report(&self, orders: &[Order], range: DateRange) -> SalesReport {
        let (start, end) = self.get_date_range(range);
        let filtered = self.filter_orders_by_date(orders, start, end);
        let paid_orders = self.get_paid_orders(&filtered);

        let total_sales: Decimal = paid_orders.iter().map(|o| o.total).sum();
        let total_shipping: Decimal = paid_orders.iter().map(|o| o.shipping_total).sum();
        let total_tax: Decimal = paid_orders.iter().map(|o| o.total_tax).sum();
        let total_discount: Decimal = paid_orders.iter().map(|o| o.discount_total).sum();
        let total_refunds = Decimal::ZERO; // Would sum from refunds

        let net_sales = total_sales - total_refunds;
        let total_orders = paid_orders.len() as i32;
        let total_items: i32 = paid_orders.iter()
            .flat_map(|o| &o.items)
            .map(|i| i.quantity)
            .sum();

        let average_order_value = if total_orders > 0 {
            net_sales / Decimal::from(total_orders)
        } else {
            Decimal::ZERO
        };

        // Count orders by status
        let mut orders_by_status: HashMap<OrderStatus, i32> = HashMap::new();
        for order in &filtered {
            *orders_by_status.entry(order.status).or_insert(0) += 1;
        }

        SalesReport {
            period: self.format_period(range),
            total_sales,
            net_sales,
            total_orders,
            total_items,
            total_shipping,
            total_tax,
            total_refunds,
            total_discount,
            average_order_value,
            orders_by_status,
        }
    }

    /// Generate dashboard stats
    pub fn generate_dashboard_stats(&self, orders: &[Order], range: DateRange) -> DashboardStats {
        let current_report = self.generate_sales_report(orders, range);

        // Calculate previous period for comparison
        let previous_range = self.get_previous_period(range);
        let previous_report = self.generate_sales_report(orders, previous_range);

        let net_sales_change = self.calculate_percentage_change(
            previous_report.net_sales,
            current_report.net_sales,
        );

        let orders_change = self.calculate_percentage_change(
            Decimal::from(previous_report.total_orders),
            Decimal::from(current_report.total_orders),
        );

        DashboardStats {
            net_sales: current_report.net_sales,
            orders_count: current_report.total_orders,
            items_sold: current_report.total_items,
            refunded: current_report.total_refunds,
            shipping: current_report.total_shipping,
            average_order_value: current_report.average_order_value,
            net_sales_change,
            orders_change,
        }
    }

    /// Get previous period for comparison
    fn get_previous_period(&self, range: DateRange) -> DateRange {
        let (start, end) = self.get_date_range(range);
        let duration = end - start;
        DateRange::Custom(start - duration - Duration::seconds(1), start - Duration::seconds(1))
    }

    /// Calculate percentage change
    fn calculate_percentage_change(&self, old_value: Decimal, new_value: Decimal) -> Decimal {
        if old_value == Decimal::ZERO {
            if new_value > Decimal::ZERO {
                dec!(100)
            } else {
                Decimal::ZERO
            }
        } else {
            ((new_value - old_value) / old_value * dec!(100)).round_dp(2)
        }
    }

    /// Get top selling products
    pub fn get_top_sellers(
        &self,
        orders: &[Order],
        products: &[Product],
        range: DateRange,
        limit: usize,
    ) -> Vec<TopSeller> {
        let (start, end) = self.get_date_range(range);
        let filtered = self.filter_orders_by_date(orders, start, end);
        let paid_orders = self.get_paid_orders(&filtered);

        // Aggregate sales by product
        let mut product_sales: HashMap<Uuid, (i32, Decimal)> = HashMap::new();
        for order in paid_orders {
            for item in &order.items {
                let entry = product_sales.entry(item.product_id).or_insert((0, Decimal::ZERO));
                entry.0 += item.quantity;
                entry.1 += item.total;
            }
        }

        // Sort by quantity sold
        let mut sorted: Vec<_> = product_sales.into_iter().collect();
        sorted.sort_by(|a, b| b.1.0.cmp(&a.1.0));
        sorted.truncate(limit);

        // Map to TopSeller structs
        sorted.into_iter()
            .map(|(product_id, (qty, revenue))| {
                let product_name = products.iter()
                    .find(|p| p.id == product_id)
                    .map(|p| p.name.clone())
                    .unwrap_or_else(|| "Unknown Product".to_string());

                TopSeller {
                    product_id,
                    product_name,
                    quantity_sold: qty,
                    total_revenue: revenue,
                }
            })
            .collect()
    }

    /// Get sales by date
    pub fn get_sales_by_date(
        &self,
        orders: &[Order],
        range: DateRange,
    ) -> Vec<SalesByDate> {
        let (start, end) = self.get_date_range(range);
        let filtered = self.filter_orders_by_date(orders, start, end);

        // Group by date
        let mut by_date: HashMap<String, Vec<&Order>> = HashMap::new();
        for order in &filtered {
            let date = order.created_at.format("%Y-%m-%d").to_string();
            by_date.entry(date).or_insert_with(Vec::new).push(order);
        }

        // Generate stats for each date
        let mut result: Vec<SalesByDate> = by_date.into_iter()
            .map(|(date, day_orders)| {
                let paid: Vec<_> = day_orders.iter()
                    .filter(|o| matches!(
                        o.status,
                        OrderStatus::Processing | OrderStatus::Completed | OrderStatus::OnHold
                    ))
                    .collect();

                let gross_sales: Decimal = paid.iter().map(|o| o.total).sum();
                let shipping: Decimal = paid.iter().map(|o| o.shipping_total).sum();
                let tax: Decimal = paid.iter().map(|o| o.total_tax).sum();
                let items: i32 = paid.iter()
                    .flat_map(|o| &o.items)
                    .map(|i| i.quantity)
                    .sum();

                SalesByDate {
                    date,
                    orders: paid.len() as i32,
                    items,
                    gross_sales,
                    net_sales: gross_sales, // Would subtract refunds
                    shipping,
                    tax,
                    refunds: Decimal::ZERO,
                }
            })
            .collect();

        result.sort_by(|a, b| a.date.cmp(&b.date));
        result
    }

    /// Get customer stats
    pub fn get_customer_stats(
        &self,
        orders: &[Order],
        customers: &[Customer],
    ) -> CustomerStats {
        let total_customers = customers.len() as i32;

        let customers_with_orders: std::collections::HashSet<_> = orders.iter()
            .filter_map(|o| o.customer_id)
            .collect();

        let paying_customers = customers_with_orders.len() as i32;

        // Get new customers this month
        let now = Utc::now();
        let month_start = now.date_naive().with_day(1).unwrap()
            .and_hms_opt(0, 0, 0).unwrap();
        let month_start = DateTime::from_naive_utc_and_offset(month_start, Utc);

        let new_customers_this_month = customers.iter()
            .filter(|c| c.created_at >= month_start)
            .count() as i32;

        CustomerStats {
            total_customers,
            paying_customers,
            new_customers_this_month,
        }
    }

    /// Format period for display
    fn format_period(&self, range: DateRange) -> String {
        match range {
            DateRange::Today => "Today".to_string(),
            DateRange::Yesterday => "Yesterday".to_string(),
            DateRange::ThisWeek => "This week".to_string(),
            DateRange::LastWeek => "Last week".to_string(),
            DateRange::ThisMonth => "This month".to_string(),
            DateRange::LastMonth => "Last month".to_string(),
            DateRange::ThisYear => "This year".to_string(),
            DateRange::LastYear => "Last year".to_string(),
            DateRange::Custom(start, end) => {
                format!("{} - {}", start.format("%Y-%m-%d"), end.format("%Y-%m-%d"))
            }
        }
    }

    /// Get stock report
    pub fn get_stock_report(&self, products: &[Product]) -> StockReport {
        let low_stock: Vec<_> = products.iter()
            .filter(|p| {
                p.manage_stock &&
                p.stock_quantity.map_or(false, |q| q > 0 && q <= 5)
            })
            .collect();

        let out_of_stock: Vec<_> = products.iter()
            .filter(|p| {
                p.stock_status == crate::models::product::StockStatus::OutOfStock ||
                (p.manage_stock && p.stock_quantity.map_or(false, |q| q <= 0))
            })
            .collect();

        StockReport {
            low_stock_count: low_stock.len() as i32,
            out_of_stock_count: out_of_stock.len() as i32,
            low_stock_products: low_stock.iter().map(|p| p.id).collect(),
            out_of_stock_products: out_of_stock.iter().map(|p| p.id).collect(),
        }
    }
}

/// Customer statistics
#[derive(Debug, Clone)]
pub struct CustomerStats {
    pub total_customers: i32,
    pub paying_customers: i32,
    pub new_customers_this_month: i32,
}

/// Stock report
#[derive(Debug, Clone)]
pub struct StockReport {
    pub low_stock_count: i32,
    pub out_of_stock_count: i32,
    pub low_stock_products: Vec<Uuid>,
    pub out_of_stock_products: Vec<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage_change() {
        let settings = RustCommerceSettings::default();
        let service = ReportService::new(settings);

        // 100 to 150 = 50% increase
        assert_eq!(
            service.calculate_percentage_change(dec!(100), dec!(150)),
            dec!(50)
        );

        // 100 to 50 = -50% decrease
        assert_eq!(
            service.calculate_percentage_change(dec!(100), dec!(50)),
            dec!(-50)
        );

        // 0 to 100 = 100% (avoid division by zero)
        assert_eq!(
            service.calculate_percentage_change(dec!(0), dec!(100)),
            dec!(100)
        );
    }

    #[test]
    fn test_format_period() {
        let settings = RustCommerceSettings::default();
        let service = ReportService::new(settings);

        assert_eq!(service.format_period(DateRange::Today), "Today");
        assert_eq!(service.format_period(DateRange::ThisMonth), "This month");
    }
}
