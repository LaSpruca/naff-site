-- Logic for generating team ID's
CREATE SEQUENCE team_seq START 1 INCREMENT 1;
CREATE FUNCTION new_id() RETURNS VARCHAR(7) AS $$
DECLARE hash TEXT;
BEGIN hash = upper(
    encode(
        digest(
            (nextval('team_seq'))::text::bytea,
            'sha256'
        ),
        'hex'
    )
);
RETURN concat(substr(hash, 1, 3), '-', substr(hash, 3, 3));
END $$ LANGUAGE plpgsql;
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    "name" TEXT NOT NULL,
    email TEXT NOT NULL
);
CREATE TABLE teams (
    id VARCHAR(7) PRIMARY KEY DEFAULT new_id(),
    "name" TEXT NOT NULL,
    film_name TEXT NOT NULL,
    film_description TEXT NOT NULL,
    has_file BOOLEAN NOT NULL DEFAULT false
);
CREATE TABLE user_connection (
    id SERIAL PRIMARY KEY,
    "user" TEXT NOT NULL REFERENCES users (id),
    team VARCHAR(7) NOT NULL REFERENCES teams (id)
);
CREATE FUNCTION has_team(user_id TEXT) RETURNS BOOLEAN AS $$ BEGIN return exists(
    SELECT 1
    FROM user_connection
    WHERE user_id = "user"
);
END $$ LANGUAGE plpgsql;