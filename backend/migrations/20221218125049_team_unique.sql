-- Add migration script here
CREATE UNIQUE INDEX user_connection_name_idx ON teams (lower("name"));