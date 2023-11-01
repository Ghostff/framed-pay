CREATE TABLE "api_keys"
(
    id         UUID                       NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name       VARCHAR(255)               NOT NULL,
    key        VARCHAR(255)               NOT NULL UNIQUE,
    user_id    UUID REFERENCES users (id) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE                        DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE                        DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE   NULL
);

CREATE INDEX api_keys_key_idx ON api_keys (key);
