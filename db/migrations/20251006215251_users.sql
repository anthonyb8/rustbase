-- migrate:up
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    is_verified BOOL NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(), 
    -- last_login TIMESTAMPTZ NULL
    -- authenticator_mfa_enabled BOOLEAN DEFAULT FALSE,
    -- mfa_secret TEXT,
);

-- CREATE TABLE IF NOT EXISTS refresh_tokens(
--   id SERIAL PRIMARY KEY,
--   user_id INT NOT NULL REFERENCES users(id) on DELETE CASCADE,
--   token_hash VARCHAR(64) NOT NULL UNIQUE,
--   expires_at TIMESTAMPTZ NOT NULL,
--   created_at TIMESTAMPTZ DEFAULT NOW()
-- );

-- CREATE TABLE IF NOT EXISTS recovery_codes (
--     id SERIAL PRIMARY KEY,
--     user_id INT NOT NULL REFERENCES users(id) on DELETE CASCADE,
--     code_hash VARCHAR(255) NOT NULL, 
--     created_at TIMESTAMPTZ DEFAULT NOW()
-- );



-- migrate:down
DROP TABLE user;
-- DROP TABLE recovery_codes;
-- DROP TABLE refresh_tokens;
