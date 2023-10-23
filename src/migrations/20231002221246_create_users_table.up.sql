CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users"
(
    id                   UUID         NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    first_name           VARCHAR(255) NOT NULL,
    last_name            VARCHAR(255) NOT NULL,
    email                VARCHAR(255) NOT NULL UNIQUE,
    avatar               VARCHAR      NOT NULL             DEFAULT 'default.png',
    password             VARCHAR(255) NOT NULL,
    password_reset_token VARCHAR(255) NULL,
    api_key              VARCHAR(255) NOT NULL UNIQUE,
    role                 VARCHAR(100) NOT NULL             DEFAULT 'user',
    last_logged_in_at    TIMESTAMP WITH TIME ZONE          DEFAULT NOW(),
    created_at           TIMESTAMP WITH TIME ZONE          DEFAULT NOW(),
    updated_at           TIMESTAMP WITH TIME ZONE          DEFAULT NOW()
);

CREATE INDEX users_email_idx ON users (email);
CREATE INDEX users_api_key_idx ON users (api_key);
-- Add up migration script here