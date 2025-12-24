-- RustCommerce Products Schema
-- Complete product management tables

-- ============================================================================
-- Product Types Enum
-- ============================================================================
DO $$ BEGIN
    CREATE TYPE product_type AS ENUM (
        'simple',
        'variable',
        'grouped',
        'external',
        'virtual',
        'downloadable',
        'subscription',
        'bundle',
        'booking'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE product_status AS ENUM (
        'draft',
        'pending',
        'private',
        'publish',
        'trash'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE stock_status AS ENUM (
        'instock',
        'outofstock',
        'onbackorder'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE backorder_status AS ENUM (
        'no',
        'notify',
        'yes'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- ============================================================================
-- Products Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,

    -- Basic info
    sku VARCHAR(255),
    name VARCHAR(500) NOT NULL,
    slug VARCHAR(500) NOT NULL,
    product_type product_type NOT NULL DEFAULT 'simple',
    status product_status NOT NULL DEFAULT 'draft',

    -- Description
    short_description TEXT,
    description TEXT,

    -- Pricing
    regular_price DECIMAL(19, 4),
    sale_price DECIMAL(19, 4),
    sale_price_from TIMESTAMPTZ,
    sale_price_to TIMESTAMPTZ,

    -- Tax
    tax_status VARCHAR(50) DEFAULT 'taxable', -- taxable, shipping, none
    tax_class VARCHAR(100) DEFAULT 'standard',

    -- Inventory
    manage_stock BOOLEAN DEFAULT FALSE,
    stock_quantity INTEGER DEFAULT 0,
    stock_status stock_status DEFAULT 'instock',
    backorders backorder_status DEFAULT 'no',
    low_stock_amount INTEGER,
    sold_individually BOOLEAN DEFAULT FALSE,

    -- Shipping
    weight DECIMAL(10, 4),
    length DECIMAL(10, 4),
    width DECIMAL(10, 4),
    height DECIMAL(10, 4),
    shipping_class_id UUID,

    -- Virtual/Downloadable
    is_virtual BOOLEAN DEFAULT FALSE,
    is_downloadable BOOLEAN DEFAULT FALSE,
    download_limit INTEGER DEFAULT -1,
    download_expiry INTEGER DEFAULT -1,

    -- External product
    external_url VARCHAR(1000),
    button_text VARCHAR(255),

    -- Reviews
    reviews_allowed BOOLEAN DEFAULT TRUE,
    average_rating DECIMAL(3, 2) DEFAULT 0,
    rating_count INTEGER DEFAULT 0,

    -- Visibility
    featured BOOLEAN DEFAULT FALSE,
    catalog_visibility VARCHAR(50) DEFAULT 'visible', -- visible, catalog, search, hidden

    -- Parent for variations
    parent_id UUID REFERENCES rc_products(id) ON DELETE CASCADE,
    menu_order INTEGER DEFAULT 0,

    -- Purchase tracking
    purchase_note TEXT,
    total_sales INTEGER DEFAULT 0,

    -- SEO
    meta_title VARCHAR(500),
    meta_description TEXT,
    meta_keywords TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    published_at TIMESTAMPTZ,

    UNIQUE (site_id, slug),
    UNIQUE (site_id, sku) -- SKU must be unique if provided
);

-- ============================================================================
-- Product Meta (for extensibility)
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_product_meta (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES rc_products(id) ON DELETE CASCADE,
    meta_key VARCHAR(255) NOT NULL,
    meta_value JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (product_id, meta_key)
);

-- ============================================================================
-- Product Categories
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_product_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    description TEXT,
    parent_id UUID REFERENCES rc_product_categories(id) ON DELETE SET NULL,
    image_id UUID,
    display_type VARCHAR(50) DEFAULT 'default', -- default, products, subcategories, both
    menu_order INTEGER DEFAULT 0,
    count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE (site_id, slug)
);

-- Category-Product relationship
CREATE TABLE IF NOT EXISTS rc_product_category_relationships (
    product_id UUID NOT NULL REFERENCES rc_products(id) ON DELETE CASCADE,
    category_id UUID NOT NULL REFERENCES rc_product_categories(id) ON DELETE CASCADE,
    PRIMARY KEY (product_id, category_id)
);

-- ============================================================================
-- Product Tags
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_product_tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    description TEXT,
    count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (site_id, slug)
);

-- Tag-Product relationship
CREATE TABLE IF NOT EXISTS rc_product_tag_relationships (
    product_id UUID NOT NULL REFERENCES rc_products(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES rc_product_tags(id) ON DELETE CASCADE,
    PRIMARY KEY (product_id, tag_id)
);

-- ============================================================================
-- Product Attributes (for variations)
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_product_attributes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    type VARCHAR(50) DEFAULT 'select', -- select, color, image, button, text
    order_by VARCHAR(50) DEFAULT 'menu_order', -- menu_order, name, name_num, id
    has_archives BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (site_id, slug)
);

-- Attribute Terms (values)
CREATE TABLE IF NOT EXISTS rc_product_attribute_terms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    attribute_id UUID NOT NULL REFERENCES rc_product_attributes(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    description TEXT,
    menu_order INTEGER DEFAULT 0,
    count INTEGER DEFAULT 0,
    -- For color/image type
    color VARCHAR(50),
    image_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (attribute_id, slug)
);

-- Product-Attribute relationship (which attributes a product uses)
CREATE TABLE IF NOT EXISTS rc_product_attribute_relationships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES rc_products(id) ON DELETE CASCADE,
    attribute_id UUID NOT NULL REFERENCES rc_product_attributes(id) ON DELETE CASCADE,
    position INTEGER DEFAULT 0,
    is_visible BOOLEAN DEFAULT TRUE,
    is_variation BOOLEAN DEFAULT FALSE, -- used for variations
    UNIQUE (product_id, attribute_id)
);

-- Product attribute values assigned
CREATE TABLE IF NOT EXISTS rc_product_attribute_values (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES rc_products(id) ON DELETE CASCADE,
    attribute_id UUID NOT NULL REFERENCES rc_product_attributes(id) ON DELETE CASCADE,
    term_id UUID REFERENCES rc_product_attribute_terms(id) ON DELETE CASCADE,
    custom_value VARCHAR(500), -- for custom text values
    UNIQUE (product_id, attribute_id, term_id)
);

-- ============================================================================
-- Product Variations
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_product_variations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES rc_products(id) ON DELETE CASCADE,

    -- Variation-specific data
    sku VARCHAR(255),
    status product_status NOT NULL DEFAULT 'publish',

    -- Pricing (overrides parent if set)
    regular_price DECIMAL(19, 4),
    sale_price DECIMAL(19, 4),
    sale_price_from TIMESTAMPTZ,
    sale_price_to TIMESTAMPTZ,

    -- Inventory
    manage_stock BOOLEAN,
    stock_quantity INTEGER,
    stock_status stock_status,
    backorders backorder_status,

    -- Shipping
    weight DECIMAL(10, 4),
    length DECIMAL(10, 4),
    width DECIMAL(10, 4),
    height DECIMAL(10, 4),

    -- Virtual/Downloadable
    is_virtual BOOLEAN,
    is_downloadable BOOLEAN,

    -- Display
    description TEXT,
    image_id UUID,
    menu_order INTEGER DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Variation attribute values (which combination of attributes this variation is)
CREATE TABLE IF NOT EXISTS rc_variation_attributes (
    variation_id UUID NOT NULL REFERENCES rc_product_variations(id) ON DELETE CASCADE,
    attribute_id UUID NOT NULL REFERENCES rc_product_attributes(id) ON DELETE CASCADE,
    term_id UUID REFERENCES rc_product_attribute_terms(id) ON DELETE CASCADE,
    custom_value VARCHAR(500),
    PRIMARY KEY (variation_id, attribute_id)
);

-- ============================================================================
-- Product Gallery
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_product_gallery (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES rc_products(id) ON DELETE CASCADE,
    media_id UUID NOT NULL,
    position INTEGER DEFAULT 0,
    is_featured BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Product Downloads (for downloadable products)
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_product_downloads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES rc_products(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    file_url VARCHAR(2000) NOT NULL,
    file_hash VARCHAR(64),
    download_count INTEGER DEFAULT 0,
    position INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Product Reviews
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_product_reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES rc_products(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,

    -- Review content
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    title VARCHAR(500),
    content TEXT NOT NULL,

    -- Reviewer info (for guests)
    reviewer_name VARCHAR(255),
    reviewer_email VARCHAR(255),

    -- Moderation
    status VARCHAR(50) DEFAULT 'pending', -- pending, approved, spam, trash
    verified_purchase BOOLEAN DEFAULT FALSE,

    -- Helpful votes
    helpful_count INTEGER DEFAULT 0,
    not_helpful_count INTEGER DEFAULT 0,

    -- Response
    response TEXT,
    response_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================================
-- Related/Cross-sell/Upsell Products
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_product_relationships (
    product_id UUID NOT NULL REFERENCES rc_products(id) ON DELETE CASCADE,
    related_product_id UUID NOT NULL REFERENCES rc_products(id) ON DELETE CASCADE,
    relationship_type VARCHAR(50) NOT NULL, -- related, cross_sell, upsell, grouped
    position INTEGER DEFAULT 0,
    PRIMARY KEY (product_id, related_product_id, relationship_type)
);

-- ============================================================================
-- Shipping Classes
-- ============================================================================
CREATE TABLE IF NOT EXISTS rc_shipping_classes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id UUID REFERENCES sites(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (site_id, slug)
);

-- ============================================================================
-- Indexes
-- ============================================================================
CREATE INDEX IF NOT EXISTS idx_rc_products_site ON rc_products(site_id);
CREATE INDEX IF NOT EXISTS idx_rc_products_slug ON rc_products(site_id, slug);
CREATE INDEX IF NOT EXISTS idx_rc_products_sku ON rc_products(site_id, sku);
CREATE INDEX IF NOT EXISTS idx_rc_products_type ON rc_products(product_type);
CREATE INDEX IF NOT EXISTS idx_rc_products_status ON rc_products(status);
CREATE INDEX IF NOT EXISTS idx_rc_products_parent ON rc_products(parent_id);
CREATE INDEX IF NOT EXISTS idx_rc_products_featured ON rc_products(site_id, featured) WHERE featured = TRUE;
CREATE INDEX IF NOT EXISTS idx_rc_products_price ON rc_products(regular_price, sale_price);

CREATE INDEX IF NOT EXISTS idx_rc_product_meta_key ON rc_product_meta(product_id, meta_key);
CREATE INDEX IF NOT EXISTS idx_rc_categories_parent ON rc_product_categories(parent_id);
CREATE INDEX IF NOT EXISTS idx_rc_variations_product ON rc_product_variations(product_id);
CREATE INDEX IF NOT EXISTS idx_rc_reviews_product ON rc_product_reviews(product_id, status);
CREATE INDEX IF NOT EXISTS idx_rc_reviews_rating ON rc_product_reviews(product_id, rating);

-- ============================================================================
-- Triggers
-- ============================================================================
CREATE OR REPLACE FUNCTION rc_update_product_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER rc_products_updated_at
    BEFORE UPDATE ON rc_products
    FOR EACH ROW
    EXECUTE FUNCTION rc_update_product_timestamp();

-- Update category count when products added/removed
CREATE OR REPLACE FUNCTION rc_update_category_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE rc_product_categories SET count = count + 1 WHERE id = NEW.category_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE rc_product_categories SET count = count - 1 WHERE id = OLD.category_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER rc_category_count_trigger
    AFTER INSERT OR DELETE ON rc_product_category_relationships
    FOR EACH ROW
    EXECUTE FUNCTION rc_update_category_count();

-- Update product average rating when reviews added
CREATE OR REPLACE FUNCTION rc_update_product_rating()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE rc_products
    SET
        average_rating = (
            SELECT COALESCE(AVG(rating), 0)
            FROM rc_product_reviews
            WHERE product_id = COALESCE(NEW.product_id, OLD.product_id)
            AND status = 'approved'
        ),
        rating_count = (
            SELECT COUNT(*)
            FROM rc_product_reviews
            WHERE product_id = COALESCE(NEW.product_id, OLD.product_id)
            AND status = 'approved'
        )
    WHERE id = COALESCE(NEW.product_id, OLD.product_id);
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER rc_product_rating_trigger
    AFTER INSERT OR UPDATE OR DELETE ON rc_product_reviews
    FOR EACH ROW
    EXECUTE FUNCTION rc_update_product_rating();
