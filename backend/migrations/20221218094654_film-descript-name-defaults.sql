-- Add migration script here
ALTER TABLE teams
ALTER COLUMN film_name
SET DEFAULT '',
    ALTER COLUMN film_description
SET DEFAULT '';