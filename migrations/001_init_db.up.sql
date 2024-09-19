-- Create auth_user Table
CREATE TABLE auth_user (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    premium BOOLEAN DEFAULT FALSE,
    google_id TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create user_session Table
CREATE TABLE user_session (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES auth_user(id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Tenants (Real Estate Agents)
CREATE TABLE tenants (
    id SERIAL PRIMARY KEY,
    auth_user_id TEXT NOT NULL REFERENCES auth_user(id) ON DELETE CASCADE,
    company_name VARCHAR(255),
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    phone VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create a function to set the created_at and updated_at timestamps
CREATE OR REPLACE FUNCTION set_timestamps()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        NEW.created_at = CURRENT_TIMESTAMP;
        NEW.updated_at = CURRENT_TIMESTAMP;
    ELSIF TG_OP = 'UPDATE' THEN
        NEW.updated_at = CURRENT_TIMESTAMP;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger for the tenants table
CREATE TRIGGER set_timestamps_tenants
BEFORE INSERT OR UPDATE ON tenants
FOR EACH ROW
EXECUTE FUNCTION set_timestamps();

-- Repeat the same for other tables if needed
-- For example, for the properties table:
CREATE TRIGGER set_timestamps_properties
BEFORE INSERT OR UPDATE ON properties
FOR EACH ROW
EXECUTE FUNCTION set_timestamps();

-- Properties
CREATE TABLE properties (
    id SERIAL PRIMARY KEY,
    tenant_id INTEGER NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    property_type VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL,
    price DOUBLE PRECISION NOT NULL,  -- Changed from DECIMAL(12, 2)
    currency VARCHAR(3) NOT NULL,
    bedrooms INTEGER,
    bathrooms INTEGER,
    parking_spaces INTEGER,
    total_area DOUBLE PRECISION,  -- Changed from DECIMAL(10, 2)
    built_area DOUBLE PRECISION,  -- Changed from DECIMAL(10, 2)
    year_built INTEGER,
    address VARCHAR(255),
    city VARCHAR(100),
    state VARCHAR(100),
    country VARCHAR(100),
    google_maps_url TEXT,
    amenities TEXT[],  -- Array of text to store amenities
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


-- Property Images
CREATE TABLE property_images (
    id SERIAL PRIMARY KEY,
    property_id INTEGER NOT NULL REFERENCES properties(id) ON DELETE CASCADE,
    image_url VARCHAR(255) NOT NULL,
    is_primary BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


-- Create indexes for better query performance
CREATE INDEX idx_auth_user_email ON auth_user(email);
CREATE INDEX idx_user_session_user_id ON user_session(user_id);
CREATE INDEX idx_tenants_auth_user_id ON tenants(auth_user_id);
CREATE INDEX idx_properties_tenant_id ON properties(tenant_id);
CREATE INDEX idx_property_features_property_id ON property_features(property_id);
CREATE INDEX idx_property_images_property_id ON property_images(property_id);




-- hero table
CREATE TABLE hero (
    id SERIAL PRIMARY KEY,
    tenant_id INTEGER NOT NULL UNIQUE REFERENCES tenants(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    image VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create a function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to automatically update the updated_at column
CREATE TRIGGER update_hero_updated_at
BEFORE UPDATE ON hero
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Config Table
CREATE TABLE config (
    id SERIAL PRIMARY KEY,
    tenant_id INTEGER NOT NULL UNIQUE REFERENCES tenants(id) ON DELETE CASCADE,
    logo VARCHAR(255) NOT NULL,
    color VARCHAR(7) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create a function to update the updated_at column (if not already created)
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to automatically update the updated_at column
CREATE TRIGGER update_config_updated_at
BEFORE UPDATE ON config
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Function to create default hero and config entries
CREATE OR REPLACE FUNCTION create_default_hero_and_config()
RETURNS TRIGGER AS $$
BEGIN
    -- Insert default hero
    INSERT INTO hero (tenant_id, title, description, image)
    VALUES (NEW.id, 'Welcome to Our Real Estate Agency', 'We help you find your dream home', 'https://default-hero-image-url.com');

    -- Insert default config
    INSERT INTO config (tenant_id, logo, color)
    VALUES (NEW.id, 'https://default-logo-url.com', '#000000');

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to call the function when a new tenant is created
CREATE TRIGGER create_hero_and_config_for_new_tenant
AFTER INSERT ON tenants
FOR EACH ROW
EXECUTE FUNCTION create_default_hero_and_config();

-- Create indexes for better query performance
CREATE INDEX idx_hero_tenant_id ON hero(tenant_id);
CREATE INDEX idx_config_tenant_id ON config(tenant_id);


-- Feedback Table
CREATE TABLE feedback (
    id SERIAL PRIMARY KEY,
    tenant_id INTEGER NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    property_image VARCHAR(255) NOT NULL,
    customer_image VARCHAR(255) NOT NULL,
    customer_name VARCHAR(100) NOT NULL,
    customer_review TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create a trigger to automatically update the updated_at column
CREATE TRIGGER update_feedback_updated_at
BEFORE UPDATE ON feedback
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Create an index for better query performance
CREATE INDEX idx_feedback_tenant_id ON feedback(tenant_id);



-- Create a function to set the created_at and updated_at timestamps
CREATE OR REPLACE FUNCTION set_timestamps()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        NEW.created_at = CURRENT_TIMESTAMP;
        NEW.updated_at = CURRENT_TIMESTAMP;
    ELSIF TG_OP = 'UPDATE' THEN
        NEW.updated_at = CURRENT_TIMESTAMP;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger for the properties table
CREATE TRIGGER set_timestamps_properties
BEFORE INSERT OR UPDATE ON properties
FOR EACH ROW
EXECUTE FUNCTION set_timestamps();


-- Create a trigger for the property_images table
CREATE TRIGGER set_timestamps_property_images
BEFORE INSERT OR UPDATE ON property_images
FOR EACH ROW
EXECUTE FUNCTION set_timestamps();
