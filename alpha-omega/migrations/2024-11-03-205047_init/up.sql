CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TYPE permission AS ENUM ('Admin', 'Manager', 'Collaborator', 'Viewer');
COMMENT ON TYPE permission IS 'ya know, permissions';

CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  email VARCHAR NOT NULL UNIQUE,
  username VARCHAR(50),
  first_name VARCHAR(50),
  last_name VARCHAR(50),
  permission permission NOT NULL DEFAULT 'Viewer',
  system_admin BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at);