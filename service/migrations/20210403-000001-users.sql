CREATE TABLE users (
  user_id UUID PRIMARY KEY,
  version UUID NOT NULL,
  created TIMESTAMPTZ NOT NULL,
  updated TIMESTAMPTZ NOT NULL,
  username TEXT NOT NULL,
  email TEXT NOT NULL,
  display_name TEXT NOT NULL,
  password TEXT NOT NULL
);