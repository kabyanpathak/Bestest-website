CREATE TABLE manhwa (
    manhwa_key serial PRIMARY KEY,
    name varchar(255) NOT NULL UNIQUE,
);

CREATE TABLE creator (
    creator_id serial PRIMARY KEY,
    name varchar(255) NOT NULL UNIQUE,
);

CREATE TABLE info (
    info_id serial PRIMARY KEY,
    synopsis text,
    score int NOT NULL,
    genre varchar(255)
);

--linking tables
CREATE TABLE manhwa_info (
    manhwa_key int NOT NULL,
    info_id int NOT NULL,
    author_id int NOT NULL,
    artist_id int NOT NULL
);

CREATE TABLE creator_info (
    manhwa_key int NOT NULL,
    creator_id int NOT NULL,
    is_author bit,
);

