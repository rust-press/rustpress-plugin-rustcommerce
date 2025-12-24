-- RustCommerce Orders & Customers Schema
-- Complete order management and customer data

-- ============================================================================
-- Order Status Enum
-- ============================================================================
DO $$ BEGIN
    CREATE TYPE order_status AS ENUM (
        'pending',
        'processing',
        'on_hold',
        'completed',
        'cancelled',
        'refunded',
        'failed',
        'checkout_draft'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- ============================================================================
-- Customers Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_customers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,

    -- Basic info
    email VARCHAR(255) NOT NULL,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    display_name VARCHAR(255),
    company VARCHAR(255),
    phone VARCHAR(50),

    -- Billing address
    billing_first_name VARCHAR(255),
    billing_last_name VARCHAR(255),
    billing_company VARCHAR(255),
    billing_address_1 VARCHAR(500),
    billing_address_2 VARCHAR(500),
    billing_city VARCHAR(255),
    billing_state VARCHAR(100),
    billing_postcode VARCHAR(50),
    billing_country VARCHAR(10),
    billing_email VARCHAR(255),
    billing_phone VARCHAR(50),

    -- Shipping address
    shipping_first_name VARCHAR(255),
    shipping_last_name VARCHAR(255),
    shipping_company VARCHAR(255),
    shipping_address_1 VARCHAR(500),
    shipping_address_2 VARCHAR(500),
    shipping_city VARCHAR(255),
    shipping_state VARCHAR(100),
    shipping_postcode VARCHAR(50),
    shipping_country VARCHAR(10),
    shipping_phone VARCHAR(50),

    -- Stats
    orders_count INTEGER DEFAULT 0,
    total_spent DECIMAL(19, 4) DEFAULT 0,
    average_order_value DECIMAL(19, 4) DEFAULT 0,

    -- Status
    is_paying_customer BOOLEAN DEFAULT FALSE,
    last_order_id UUID,
    last_order_date TIMESTAMPTZ,

    -- Metadata
    avatar_url VARCHAR(500),
    meta JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE (site_id, email)
);

-- Customer meta
CREATE TABLE IF NOT EXISTS rc_customer_meta (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES rc_customers(id) ON DELETE CASCADE,
    meta_key VARCHAR(255) NOT NULL,
    meta_value JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (customer_id, meta_key)
);

-- Customer sessions for cart persistence
CREATE TABLE IF NOT EXISTS rc_customer_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    session_key VARCHAR(100) NOT NULL,
    customer_id UUID REFERENCES rc_customers(id) ON DELETE CASCADE,
    session_value JSONB NOT NULL DEFAULT '{}',
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (site_id, session_key)
);

-- ============================================================================
-- Orders Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,

    -- Order number (display)
    order_number VARCHAR(50) NOT NULL,

    -- Customer
    customer_id UUID REFERENCES rc_customers(id) ON DELETE SET NULL,
    customer_ip_address VARCHAR(100),
    customer_user_agent TEXT,

    -- Status
    status order_status NOT NULL DEFAULT 'pending',
    parent_id UUID REFERENCES rc_orders(id) ON DELETE SET NULL, -- for refunds

    -- Currency
    currency VARCHAR(10) NOT NULL DEFAULT 'USD',
    currency_symbol VARCHAR(10) DEFAULT '$',

    -- Prices (all stored in base currency)
    prices_include_tax BOOLEAN DEFAULT FALSE,

    -- Totals
    discount_total DECIMAL(19, 4) DEFAULT 0,
    discount_tax DECIMAL(19, 4) DEFAULT 0,
    shipping_total DECIMAL(19, 4) DEFAULT 0,
    shipping_tax DECIMAL(19, 4) DEFAULT 0,
    cart_tax DECIMAL(19, 4) DEFAULT 0,
    total DECIMAL(19, 4) NOT NULL DEFAULT 0,
    total_tax DECIMAL(19, 4) DEFAULT 0,

    -- Billing
    billing_first_name VARCHAR(255),
    billing_last_name VARCHAR(255),
    billing_company VARCHAR(255),
    billing_address_1 VARCHAR(500),
    billing_address_2 VARCHAR(500),
    billing_city VARCHAR(255),
    billing_state VARCHAR(100),
    billing_postcode VARCHAR(50),
    billing_country VARCHAR(10),
    billing_email VARCHAR(255),
    billing_phone VARCHAR(50),

    -- Shipping
    shipping_first_name VARCHAR(255),
    shipping_last_name VARCHAR(255),
    shipping_company VARCHAR(255),
    shipping_address_1 VARCHAR(500),
    shipping_address_2 VARCHAR(500),
    shipping_city VARCHAR(255),
    shipping_state VARCHAR(100),
    shipping_postcode VARCHAR(50),
    shipping_country VARCHAR(10),
    shipping_phone VARCHAR(50),

    -- Payment
    payment_method VARCHAR(100),
    payment_method_title VARCHAR(255),
    transaction_id VARCHAR(255),

    -- Shipping method
    shipping_method VARCHAR(255),
    shipping_method_title VARCHAR(255),

    -- Notes
    customer_note TEXT,

    -- Dates
    date_paid TIMESTAMPTZ,
    date_completed TIMESTAMPTZ,

    -- Cart hash (for duplicate detection)
    cart_hash VARCHAR(64),

    -- Metadata
    meta JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================================
-- Order Items
-- ============================================================================
DO $$ BEGIN
    CREATE TYPE order_item_type AS ENUM (
        'line_item',
        'shipping',
        'tax',
        'coupon',
        'fee'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS rc_order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES rc_orders(id) ON DELETE CASCADE,
    item_type order_item_type NOT NULL DEFAULT 'line_item',
    name VARCHAR(500) NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    subtotal DECIMAL(19, 4) DEFAULT 0,
    subtotal_tax DECIMAL(19, 4) DEFAULT 0,
    total DECIMAL(19, 4) DEFAULT 0,
    total_tax DECIMAL(19, 4) DEFAULT 0,

    -- For line items
    product_id UUID REFERENCES rc_products(id) ON DELETE SET NULL,
    variation_id UUID REFERENCES rc_product_variations(id) ON DELETE SET NULL,
    sku VARCHAR(255),

    -- For tax items
    tax_rate_id UUID,
    tax_class VARCHAR(100),

    -- For shipping items
    method_id VARCHAR(100),
    instance_id VARCHAR(100),

    -- For coupon items
    coupon_code VARCHAR(100),
    discount_type VARCHAR(50),

    -- Metadata
    meta JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Order item meta
CREATE TABLE IF NOT EXISTS rc_order_item_meta (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_item_id UUID NOT NULL REFERENCES rc_order_items(id) ON DELETE CASCADE,
    meta_key VARCHAR(255) NOT NULL,
    meta_value JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Order Notes
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_order_notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES rc_orders(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    is_customer_note BOOLEAN DEFAULT FALSE,
    added_by_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Order Refunds
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_order_refunds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES rc_orders(id) ON DELETE CASCADE,
    amount DECIMAL(19, 4) NOT NULL,
    reason TEXT,
    refunded_by UUID REFERENCES users(id) ON DELETE SET NULL,
    refunded_payment BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Refund items
CREATE TABLE IF NOT EXISTS rc_order_refund_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    refund_id UUID NOT NULL REFERENCES rc_order_refunds(id) ON DELETE CASCADE,
    order_item_id UUID NOT NULL REFERENCES rc_order_items(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL,
    refund_total DECIMAL(19, 4) NOT NULL,
    refund_tax DECIMAL(19, 4) DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Downloadable Product Permissions
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_download_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES rc_orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES rc_products(id) ON DELETE CASCADE,
    download_id UUID NOT NULL REFERENCES rc_product_downloads(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    user_email VARCHAR(255),

    -- Access control
    downloads_remaining INTEGER, -- NULL = unlimited
    access_expires TIMESTAMPTZ, -- NULL = never
    download_count INTEGER DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Download log
CREATE TABLE IF NOT EXISTS rc_download_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    permission_id UUID NOT NULL REFERENCES rc_download_permissions(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    user_ip_address VARCHAR(100),
    user_agent TEXT,
    downloaded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Shopping Cart (for persistent carts)
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_cart (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    session_key VARCHAR(100),
    customer_id UUID REFERENCES rc_customers(id) ON DELETE CASCADE,

    -- Cart contents (JSON for flexibility)
    cart_contents JSONB NOT NULL DEFAULT '{}',
    cart_totals JSONB DEFAULT '{}',

    -- Applied coupons
    applied_coupons JSONB DEFAULT '[]',

    -- Status
    is_empty BOOLEAN DEFAULT TRUE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '30 days')
);

-- ============================================================================
-- Coupons
-- ============================================================================
DO $$ BEGIN
    CREATE TYPE discount_type AS ENUM (
        'percent',
        'fixed_cart',
        'fixed_product',
        'percent_product'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS rc_coupons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,

    -- Basic info
    code VARCHAR(100) NOT NULL,
    description TEXT,
    status VARCHAR(50) DEFAULT 'publish', -- publish, draft, pending, trash

    -- Discount
    discount_type discount_type NOT NULL DEFAULT 'fixed_cart',
    amount DECIMAL(19, 4) NOT NULL DEFAULT 0,

    -- Usage restrictions
    individual_use BOOLEAN DEFAULT FALSE,
    product_ids UUID[] DEFAULT '{}',
    excluded_product_ids UUID[] DEFAULT '{}',
    category_ids UUID[] DEFAULT '{}',
    excluded_category_ids UUID[] DEFAULT '{}',

    -- Usage limits
    usage_limit INTEGER, -- NULL = unlimited
    usage_limit_per_user INTEGER,
    limit_usage_to_x_items INTEGER,
    usage_count INTEGER DEFAULT 0,

    -- Amount restrictions
    minimum_amount DECIMAL(19, 4),
    maximum_amount DECIMAL(19, 4),

    -- Free shipping
    free_shipping BOOLEAN DEFAULT FALSE,

    -- Dates
    date_expires TIMESTAMPTZ,

    -- Exclude sale items
    exclude_sale_items BOOLEAN DEFAULT FALSE,

    -- Metadata
    meta JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE (site_id, code)
);

-- Coupon usage tracking
CREATE TABLE IF NOT EXISTS rc_coupon_usage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    coupon_id UUID NOT NULL REFERENCES rc_coupons(id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES rc_orders(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES rc_customers(id) ON DELETE SET NULL,
    used_by_email VARCHAR(255),
    discount_amount DECIMAL(19, 4) NOT NULL,
    used_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Wishlists
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_wishlists (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES rc_customers(id) ON DELETE CASCADE,
    session_key VARCHAR(100),
    name VARCHAR(255) DEFAULT 'Wishlist',
    is_public BOOLEAN DEFAULT FALSE,
    share_key VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS rc_wishlist_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wishlist_id UUID NOT NULL REFERENCES rc_wishlists(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES rc_products(id) ON DELETE CASCADE,
    variation_id UUID REFERENCES rc_product_variations(id) ON DELETE CASCADE,
    quantity INTEGER DEFAULT 1,
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (wishlist_id, product_id, variation_id)
);

-- ============================================================================
-- Indexes
-- ============================================================================
CREATE INDEX IF NOT EXISTS idx_rc_customers_site ON rc_customers(site_id);
CREATE INDEX IF NOT EXISTS idx_rc_customers_email ON rc_customers(site_id, email);
CREATE INDEX IF NOT EXISTS idx_rc_customers_user ON rc_customers(user_id);

CREATE INDEX IF NOT EXISTS idx_rc_orders_site ON rc_orders(site_id);
CREATE INDEX IF NOT EXISTS idx_rc_orders_customer ON rc_orders(customer_id);
CREATE INDEX IF NOT EXISTS idx_rc_orders_status ON rc_orders(status);
CREATE INDEX IF NOT EXISTS idx_rc_orders_date ON rc_orders(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_rc_orders_number ON rc_orders(site_id, order_number);

CREATE INDEX IF NOT EXISTS idx_rc_order_items_order ON rc_order_items(order_id);
CREATE INDEX IF NOT EXISTS idx_rc_order_items_product ON rc_order_items(product_id);

CREATE INDEX IF NOT EXISTS idx_rc_coupons_code ON rc_coupons(site_id, code);
CREATE INDEX IF NOT EXISTS idx_rc_coupons_expires ON rc_coupons(date_expires) WHERE date_expires IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_rc_cart_session ON rc_cart(site_id, session_key);
CREATE INDEX IF NOT EXISTS idx_rc_cart_customer ON rc_cart(customer_id);
CREATE INDEX IF NOT EXISTS idx_rc_cart_expires ON rc_cart(expires_at);

CREATE INDEX IF NOT EXISTS idx_rc_sessions_expires ON rc_customer_sessions(expires_at);

-- ============================================================================
-- Triggers
-- ============================================================================

-- Update order timestamps
CREATE TRIGGER rc_orders_updated_at
    BEFORE UPDATE ON rc_orders
    FOR EACH ROW
    EXECUTE FUNCTION rc_update_product_timestamp();

-- Update customer stats when order completed
CREATE OR REPLACE FUNCTION rc_update_customer_stats()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status = 'completed' AND (OLD.status IS NULL OR OLD.status != 'completed') THEN
        UPDATE rc_customers
        SET
            orders_count = orders_count + 1,
            total_spent = total_spent + NEW.total,
            average_order_value = (total_spent + NEW.total) / (orders_count + 1),
            is_paying_customer = TRUE,
            last_order_id = NEW.id,
            last_order_date = NEW.created_at
        WHERE id = NEW.customer_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER rc_customer_stats_trigger
    AFTER INSERT OR UPDATE ON rc_orders
    FOR EACH ROW
    EXECUTE FUNCTION rc_update_customer_stats();

-- Update coupon usage count
CREATE OR REPLACE FUNCTION rc_update_coupon_usage_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE rc_coupons SET usage_count = usage_count + 1 WHERE id = NEW.coupon_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE rc_coupons SET usage_count = usage_count - 1 WHERE id = OLD.coupon_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER rc_coupon_usage_trigger
    AFTER INSERT OR DELETE ON rc_coupon_usage
    FOR EACH ROW
    EXECUTE FUNCTION rc_update_coupon_usage_count();

-- Sequence for order numbers
CREATE SEQUENCE IF NOT EXISTS rc_order_number_seq START WITH 1000;
