CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    first_name VARCHAR(50), 
    last_name VARCHAR(50),
    is_verified BOOL NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS auth_providers (
    id SERIAL PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL,
    password_hash VARCHAR(255),
    access_token TEXT,
    refresh_token TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW() 
);

CREATE UNIQUE INDEX idx_auth_provider ON auth_providers(user_id, provider);

CREATE TABLE IF NOT EXISTS linked_accounts (
  id SERIAL PRIMARY KEY,
  user_id UUID REFERENCES users(id) ON DELETE CASCADE,
  provider TEXT NOT NULL,               -- 'google', 'notion', 'slack', etc.
  access_token TEXT,
  refresh_token TEXT,
  expires_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX idx_linked_accounts ON linked_accounts(user_id, provider);

CREATE TABLE IF NOT EXISTS objects (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) on DELETE CASCADE,
    key VARCHAR(255) NOT NULL,  -- S3 key
    filename VARCHAR(255) NOT NULL  -- Original filename
    -- content_type VARCHAR(100),
    -- size_bytes BIGINT,
    -- visibility VARCHAR(20) DEFAULT 'private',  -- 'public' | 'private'
    -- created_at TIMESTAMP DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_user_files ON objects(user_id, filename);

CREATE TABLE IF NOT EXISTS refresh_tokens(
  id SERIAL PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES users(id) on DELETE CASCADE,
  token_hash VARCHAR(64) NOT NULL UNIQUE,
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- CREATE TABLE IF NOT EXISTS recovery_codes (
--     id SERIAL PRIMARY KEY,
--     user_id INT NOT NULL REFERENCES users(id) on DELETE CASCADE,
--     code_hash VARCHAR(255) NOT NULL, 
--     created_at TIMESTAMPTZ DEFAULT NOW()
-- );


