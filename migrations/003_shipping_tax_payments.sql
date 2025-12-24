-- RustCommerce Shipping, Tax & Payments Schema

-- ============================================================================
-- Shipping Zones
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_shipping_zones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    zone_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Shipping zone locations (countries/states/postcodes)
CREATE TABLE IF NOT EXISTS rc_shipping_zone_locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    zone_id UUID NOT NULL REFERENCES rc_shipping_zones(id) ON DELETE CASCADE,
    location_code VARCHAR(100) NOT NULL, -- country code, state code, postcode, or postcode wildcard
    location_type VARCHAR(50) NOT NULL, -- country, state, postcode
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Shipping methods per zone
CREATE TABLE IF NOT EXISTS rc_shipping_zone_methods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    zone_id UUID NOT NULL REFERENCES rc_shipping_zones(id) ON DELETE CASCADE,
    method_id VARCHAR(100) NOT NULL, -- flat_rate, free_shipping, local_pickup, etc.
    method_order INTEGER DEFAULT 0,
    is_enabled BOOLEAN DEFAULT TRUE,
    settings JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Shipping Methods Configuration
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_shipping_methods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    method_id VARCHAR(100) NOT NULL,
    method_title VARCHAR(255) NOT NULL,
    method_description TEXT,

    -- Flat rate settings
    cost DECIMAL(19, 4) DEFAULT 0,
    cost_per_item DECIMAL(19, 4) DEFAULT 0,

    -- Free shipping settings
    min_amount DECIMAL(19, 4),
    requires_coupon BOOLEAN DEFAULT FALSE,

    -- Class costs (for shipping classes)
    class_costs JSONB DEFAULT '{}',
    no_class_cost DECIMAL(19, 4) DEFAULT 0,

    -- Calculation type
    calc_type VARCHAR(50) DEFAULT 'order', -- per_order, per_class, per_item

    -- Tax
    tax_status VARCHAR(50) DEFAULT 'taxable',

    is_enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE (site_id, method_id)
);

-- ============================================================================
-- Tax Rates
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_tax_rates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,

    -- Location
    country VARCHAR(10) NOT NULL DEFAULT '',
    state VARCHAR(100) DEFAULT '',
    postcode VARCHAR(100) DEFAULT '',
    city VARCHAR(255) DEFAULT '',

    -- Rate
    rate DECIMAL(8, 4) NOT NULL DEFAULT 0,
    name VARCHAR(255) NOT NULL DEFAULT 'Tax',

    -- Priority (for compound taxes)
    priority INTEGER DEFAULT 1,
    compound BOOLEAN DEFAULT FALSE,
    shipping BOOLEAN DEFAULT TRUE,

    -- Order
    tax_order INTEGER DEFAULT 0,

    -- Tax class
    tax_class VARCHAR(100) DEFAULT 'standard',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Tax rate locations (for more specific targeting)
CREATE TABLE IF NOT EXISTS rc_tax_rate_locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tax_rate_id UUID NOT NULL REFERENCES rc_tax_rates(id) ON DELETE CASCADE,
    location_code VARCHAR(100) NOT NULL,
    location_type VARCHAR(50) NOT NULL, -- postcode, city
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Tax classes
CREATE TABLE IF NOT EXISTS rc_tax_classes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (site_id, slug)
);

-- ============================================================================
-- Payment Gateways
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_payment_gateways (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    gateway_id VARCHAR(100) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,

    -- Status
    is_enabled BOOLEAN DEFAULT FALSE,
    method_order INTEGER DEFAULT 0,

    -- Settings (encrypted sensitive data)
    settings JSONB DEFAULT '{}',

    -- Supported features
    supports JSONB DEFAULT '["products"]', -- products, subscriptions, refunds, etc.

    -- Countries
    countries TEXT[] DEFAULT '{}', -- Empty = all countries

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE (site_id, gateway_id)
);

-- Payment tokens (saved payment methods)
CREATE TABLE IF NOT EXISTS rc_payment_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES rc_customers(id) ON DELETE CASCADE,
    gateway_id VARCHAR(100) NOT NULL,

    -- Token data
    token VARCHAR(500) NOT NULL, -- Gateway's token identifier
    token_type VARCHAR(50) NOT NULL, -- CC, eCheck, etc.

    -- Display info (non-sensitive)
    last_four VARCHAR(4),
    expiry_month VARCHAR(2),
    expiry_year VARCHAR(4),
    card_type VARCHAR(50), -- visa, mastercard, etc.

    -- Default
    is_default BOOLEAN DEFAULT FALSE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ
);

-- Payment transactions
CREATE TABLE IF NOT EXISTS rc_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES rc_orders(id) ON DELETE CASCADE,

    -- Transaction info
    transaction_id VARCHAR(255) NOT NULL,
    gateway_id VARCHAR(100) NOT NULL,
    transaction_type VARCHAR(50) NOT NULL, -- payment, refund, void, capture

    -- Amounts
    amount DECIMAL(19, 4) NOT NULL,
    currency VARCHAR(10) NOT NULL,

    -- Status
    status VARCHAR(50) NOT NULL, -- pending, completed, failed, refunded

    -- Raw response from gateway
    gateway_response JSONB DEFAULT '{}',

    -- Error info
    error_code VARCHAR(100),
    error_message TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Reports & Analytics Tables
-- ============================================================================

-- Order stats (denormalized for fast reporting)
CREATE TABLE IF NOT EXISTS rc_order_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES rc_orders(id) ON DELETE CASCADE,

    -- Date dimensions
    date_created DATE NOT NULL,
    date_paid DATE,
    date_completed DATE,

    -- Customer info
    customer_id UUID,
    customer_type VARCHAR(50), -- new, returning

    -- Totals
    num_items_sold INTEGER DEFAULT 0,
    gross_total DECIMAL(19, 4) DEFAULT 0,
    tax_total DECIMAL(19, 4) DEFAULT 0,
    shipping_total DECIMAL(19, 4) DEFAULT 0,
    net_total DECIMAL(19, 4) DEFAULT 0,

    -- Status
    status VARCHAR(50) NOT NULL,

    -- Returning
    returning_customer BOOLEAN DEFAULT FALSE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (order_id)
);

-- Product stats (denormalized)
CREATE TABLE IF NOT EXISTS rc_product_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES rc_products(id) ON DELETE CASCADE,

    -- Period (for aggregation)
    date DATE NOT NULL,

    -- Stats
    items_sold INTEGER DEFAULT 0,
    gross_revenue DECIMAL(19, 4) DEFAULT 0,
    orders_count INTEGER DEFAULT 0,

    -- Running totals
    total_items_sold INTEGER DEFAULT 0,
    total_revenue DECIMAL(19, 4) DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (product_id, date)
);

-- Customer stats
CREATE TABLE IF NOT EXISTS rc_customer_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES rc_customers(id) ON DELETE CASCADE,

    -- Period
    date DATE NOT NULL,

    -- Stats
    orders_count INTEGER DEFAULT 0,
    total_spent DECIMAL(19, 4) DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (customer_id, date)
);

-- ============================================================================
-- Store Credit & Gift Cards
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_store_credit (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES rc_customers(id) ON DELETE CASCADE,

    balance DECIMAL(19, 4) NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE (site_id, customer_id)
);

CREATE TABLE IF NOT EXISTS rc_store_credit_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_credit_id UUID NOT NULL REFERENCES rc_store_credit(id) ON DELETE CASCADE,

    amount DECIMAL(19, 4) NOT NULL,
    type VARCHAR(50) NOT NULL, -- credit, debit
    reason VARCHAR(255),
    order_id UUID REFERENCES rc_orders(id) ON DELETE SET NULL,

    balance_after DECIMAL(19, 4) NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS rc_gift_cards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,

    code VARCHAR(50) NOT NULL,
    initial_balance DECIMAL(19, 4) NOT NULL,
    current_balance DECIMAL(19, 4) NOT NULL,

    -- Purchaser
    purchaser_email VARCHAR(255),
    purchaser_name VARCHAR(255),

    -- Recipient
    recipient_email VARCHAR(255),
    recipient_name VARCHAR(255),
    personal_message TEXT,

    -- Delivery
    send_at TIMESTAMPTZ,
    sent_at TIMESTAMPTZ,

    -- Status
    is_active BOOLEAN DEFAULT TRUE,
    expires_at TIMESTAMPTZ,

    -- Order
    order_id UUID REFERENCES rc_orders(id) ON DELETE SET NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (site_id, code)
);

CREATE TABLE IF NOT EXISTS rc_gift_card_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    gift_card_id UUID NOT NULL REFERENCES rc_gift_cards(id) ON DELETE CASCADE,

    amount DECIMAL(19, 4) NOT NULL,
    type VARCHAR(50) NOT NULL, -- purchase, redemption, refund
    order_id UUID REFERENCES rc_orders(id) ON DELETE SET NULL,

    balance_after DECIMAL(19, 4) NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Points & Rewards
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_points_balance (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES rc_customers(id) ON DELETE CASCADE,

    points_balance INTEGER NOT NULL DEFAULT 0,
    points_earned INTEGER NOT NULL DEFAULT 0,
    points_redeemed INTEGER NOT NULL DEFAULT 0,
    points_expired INTEGER NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE (site_id, customer_id)
);

CREATE TABLE IF NOT EXISTS rc_points_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    points_balance_id UUID NOT NULL REFERENCES rc_points_balance(id) ON DELETE CASCADE,

    points INTEGER NOT NULL,
    type VARCHAR(50) NOT NULL, -- earned, redeemed, expired, adjusted
    reason VARCHAR(255),

    -- References
    order_id UUID REFERENCES rc_orders(id) ON DELETE SET NULL,
    review_id UUID REFERENCES rc_product_reviews(id) ON DELETE SET NULL,

    balance_after INTEGER NOT NULL,
    expires_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Indexes
-- ============================================================================
CREATE INDEX IF NOT EXISTS idx_rc_shipping_zones_site ON rc_shipping_zones(site_id);
CREATE INDEX IF NOT EXISTS idx_rc_shipping_zone_locations ON rc_shipping_zone_locations(zone_id);
CREATE INDEX IF NOT EXISTS idx_rc_tax_rates_location ON rc_tax_rates(country, state, postcode, city);
CREATE INDEX IF NOT EXISTS idx_rc_tax_rates_class ON rc_tax_rates(tax_class);
CREATE INDEX IF NOT EXISTS idx_rc_payment_gateways_enabled ON rc_payment_gateways(site_id, is_enabled);
CREATE INDEX IF NOT EXISTS idx_rc_payment_tokens_customer ON rc_payment_tokens(customer_id);
CREATE INDEX IF NOT EXISTS idx_rc_transactions_order ON rc_transactions(order_id);
CREATE INDEX IF NOT EXISTS idx_rc_order_stats_date ON rc_order_stats(site_id, date_created);
CREATE INDEX IF NOT EXISTS idx_rc_product_stats_date ON rc_product_stats(product_id, date);
CREATE INDEX IF NOT EXISTS idx_rc_gift_cards_code ON rc_gift_cards(site_id, code);
CREATE INDEX IF NOT EXISTS idx_rc_points_customer ON rc_points_balance(customer_id);

-- ============================================================================
-- Default Data
-- ============================================================================

-- Insert default "Rest of World" shipping zone
-- INSERT INTO rc_shipping_zones (id, site_id, name, zone_order)
-- SELECT gen_random_uuid(), NULL, 'Rest of World', 0
-- WHERE NOT EXISTS (SELECT 1 FROM rc_shipping_zones WHERE name = 'Rest of World' AND site_id IS NULL);

-- Insert default tax classes
-- INSERT INTO rc_tax_classes (site_id, name, slug) VALUES
-- (NULL, 'Standard', 'standard'),
-- (NULL, 'Reduced Rate', 'reduced-rate'),
-- (NULL, 'Zero Rate', 'zero-rate')
-- ON CONFLICT DO NOTHING;
