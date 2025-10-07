-- Enable UUID support
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- =====================
-- ENUM TYPES (Idempotent Creation)
-- =====================
DO $$ BEGIN CREATE TYPE user_role AS ENUM ('admin', 'regular');
EXCEPTION
WHEN duplicate_object THEN null;
END $$;
DO $$ BEGIN CREATE TYPE user_status AS ENUM ('active', 'disactive');
EXCEPTION
WHEN duplicate_object THEN null;
END $$;
DO $$ BEGIN CREATE TYPE apartment_condition AS ENUM ('new', 'repaired', 'old');
EXCEPTION
WHEN duplicate_object THEN null;
END $$;
DO $$ BEGIN CREATE TYPE sale_type AS ENUM ('buy', 'rent');
EXCEPTION
WHEN duplicate_object THEN null;
END $$;
DO $$ BEGIN CREATE TYPE provider AS ENUM ('google', 'github', 'email');
EXCEPTION
WHEN duplicate_object THEN null;
END $$;
-- =====================
-- OAUTH USERS
-- =====================
CREATE TABLE IF NOT EXISTS oauth_users (
    id VARCHAR(255) PRIMARY KEY,
    provider provider NOT NULL,
    username VARCHAR(50),
    full_name VARCHAR(50),
    email VARCHAR(100),
    phone_number VARCHAR(50),
    password TEXT,
    picture TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_oauth_email UNIQUE(email),
    CONSTRAINT uq_oauth_phone UNIQUE(phone_number)
);
-- =====================
-- USERS
-- =====================
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    full_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    phone_number VARCHAR(50) NOT NULL,
    password TEXT NOT NULL,
    picture TEXT,
    role user_role NOT NULL DEFAULT 'regular',
    status user_status NOT NULL DEFAULT 'active',
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    oauth_user_id VARCHAR(255) REFERENCES oauth_users(id) ON DELETE
    SET NULL,
        -- Changed to allow NULL
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- =====================
-- APARTMENTS
-- =====================
CREATE TABLE IF NOT EXISTS apartments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    rooms INTEGER,
    area DOUBLE PRECISION,
    floor INTEGER,
    has_elevator BOOLEAN,
    condition apartment_condition NOT NULL,
    sale_type sale_type NOT NULL,
    requirements TEXT,
    has_garden BOOLEAN,
    distance_to_kindergarten DOUBLE PRECISION,
    distance_to_school DOUBLE PRECISION,
    distance_to_hospital DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- =====================
-- APARTMENT PICTURES
-- =====================
CREATE TABLE IF NOT EXISTS apartment_pictures (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    apartment_id UUID NOT NULL REFERENCES apartments(id) ON DELETE CASCADE,
    url TEXT NOT NULL,
    is_primary BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- =====================
-- ADDRESSES
-- =====================
CREATE TABLE IF NOT EXISTS addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    apartment_id UUID NOT NULL UNIQUE REFERENCES apartments(id) ON DELETE CASCADE,
    street_address TEXT NOT NULL,
    city VARCHAR(100) NOT NULL,
    state_or_region VARCHAR(100) NOT NULL,
    county_or_district VARCHAR(100),
    postal_code VARCHAR(20) NOT NULL,
    country VARCHAR(100) NOT NULL,
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- =====================
-- LISTINGS
-- =====================
CREATE TABLE IF NOT EXISTS listings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    apartment_id UUID NOT NULL REFERENCES apartments(id) ON DELETE CASCADE,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    price DOUBLE PRECISION NOT NULL,
    available_from TIMESTAMPTZ,
    available_to TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- =====================
-- FAVORITES
-- =====================
CREATE TABLE IF NOT EXISTS favorites (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, listing_id)
);
-- -- Enable UUID support
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- -- =====================
-- -- ENUM TYPES
-- -- =====================
-- CREATE TYPE user_role AS ENUM ('admin', 'regular');
-- CREATE TYPE user_status AS ENUM ('active', 'disactive');
-- CREATE TYPE apartment_condition AS ENUM ('new', 'repaired', 'old');
-- CREATE TYPE sale_type AS ENUM ('buy', 'rent');
-- CREATE TYPE provider AS ENUM ('google', 'github', 'email');
-- -- =====================
-- -- OAUTH USERS
-- -- =====================
-- CREATE TABLE oauth_users (
--     id VARCHAR(255) PRIMARY KEY,
--     provider provider NOT NULL,
--     username VARCHAR(50),
--     full_name VARCHAR(50),
--     email VARCHAR(100),
--     phone_number VARCHAR(50),
--     password TEXT,
--     picture TEXT,
--     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
--     CONSTRAINT uq_oauth_email UNIQUE(email),
--     CONSTRAINT uq_oauth_phone UNIQUE(phone_number)
-- );
-- -- =====================
-- -- USERS
-- -- =====================
-- CREATE TABLE users (
--     id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
--     full_name VARCHAR(100) NOT NULL,
--     email VARCHAR(255) NOT NULL UNIQUE,
--     phone_number VARCHAR(50) NOT NULL,
--     password TEXT NOT NULL,
--     picture TEXT,
--     role user_role NOT NULL DEFAULT 'regular',
--     status user_status NOT NULL DEFAULT 'active',
--     email_verified BOOLEAN NOT NULL DEFAULT FALSE,
--     oauth_user_id VARCHAR(255) NOT NULL REFERENCES oauth_users(id) ON DELETE CASCADE,
--     created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
--     updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
-- );
-- -- =====================
-- -- APARTMENTS
-- -- =====================
-- CREATE TABLE apartments (
--     id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
--     title VARCHAR(255) NOT NULL,
--     description TEXT,
--     rooms INTEGER,
--     area DOUBLE PRECISION,
--     floor INTEGER,
--     has_elevator BOOLEAN,
--     condition apartment_condition NOT NULL,
--     sale_type sale_type NOT NULL,
--     requirements TEXT,
--     has_garden BOOLEAN,
--     distance_to_kindergarten DOUBLE PRECISION,
--     distance_to_school DOUBLE PRECISION,
--     distance_to_hospital DOUBLE PRECISION,
--     created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
--     updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
-- );
-- -- =====================
-- -- APARTMENT PICTURES
-- -- =====================
-- CREATE TABLE apartment_pictures (
--     id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
--     apartment_id UUID NOT NULL REFERENCES apartments(id) ON DELETE CASCADE,
--     url TEXT NOT NULL,
--     is_primary BOOLEAN DEFAULT FALSE,
--     created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
-- );
-- -- =====================
-- -- ADDRESSES
-- -- =====================
-- CREATE TABLE addresses (
--     id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
--     apartment_id UUID NOT NULL REFERENCES apartments(id) ON DELETE CASCADE,
--     street_address TEXT NOT NULL,
--     city VARCHAR(100) NOT NULL,
--     state_or_region VARCHAR(100) NOT NULL,
--     county_or_district VARCHAR(100),
--     postal_code VARCHAR(20) NOT NULL,
--     country VARCHAR(100) NOT NULL,
--     latitude DOUBLE PRECISION,
--     longitude DOUBLE PRECISION,
--     created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
--     updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
-- );
-- -- =====================
-- -- LISTINGS
-- -- =====================
-- CREATE TABLE listings (
--     id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
--     apartment_id UUID NOT NULL REFERENCES apartments(id) ON DELETE CASCADE,
--     owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
--     price DOUBLE PRECISION NOT NULL,
--     available_from TIMESTAMPTZ,
--     available_to TIMESTAMPTZ,
--     created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
--     updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
-- );
-- -- =====================
-- -- FAVORITES
-- -- =====================
-- CREATE TABLE favorites (
--     id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
--     user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
--     listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
--     created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
--     updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
--     UNIQUE (user_id, listing_id)
-- );
-- ALTER TABLE users
-- ADD CONSTRAINT uq_user_oauth UNIQUE (oauth_user_id);
-- =====================
-- GOOGLE OAUTH USERS
-- =====================
-- CREATE TABLE google_oauth_users (
--     sub TEXT PRIMARY KEY,
--     email VARCHAR(100),
--     email_verified BOOLEAN NOT NULL DEFAULT false,
--     family_name VARCHAR(100),
--     given_name VARCHAR(100),
--     phone_number VARCHAR(50),
--     name VARCHAR(100),
--     picture TEXT,
--     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
-- );
-- =====================
-- GITHUB OAUTH USERS
-- =====================
-- CREATE TABLE github_oauth_users (
--     id BIGINT PRIMARY KEY,
--     login VARCHAR(100) NOT NULL,
--     avatar_url TEXT NOT NULL,
--     name VARCHAR(100),
--     email VARCHAR(100),
--     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
-- );