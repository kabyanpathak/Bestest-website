-- Create the 'manhwa' table to store basic manhwa information
CREATE TABLE manhwa (
    manhwa_id serial PRIMARY KEY, -- Unique identifier for each manhwa, automatically generated
    name varchar(255) NOT NULL UNIQUE, -- The title of the manhwa, must be unique
    synopsis text -- A longer text field for the manhwa synopsis
);

-- Create the 'authors' table to store author information
CREATE TABLE authors (
    author_id serial PRIMARY KEY, -- Unique identifier for each author, automatically generated
    name varchar(255) NOT NULL UNIQUE -- The name of the author, must be unique
);

-- Create the 'genres' table to store genre information
CREATE TABLE genres (
    genre_id serial PRIMARY KEY, -- Unique identifier for each genre, automatically generated
    name varchar(255) NOT NULL UNIQUE -- The name of the genre, must be unique
);

-- Create a linking table for the many-to-many relationship between manhwa and authors
-- A manhwa can have multiple authors, and an author can write multiple manhwa
CREATE TABLE manhwa_authors (
    manhwa_id int NOT NULL, -- Foreign key referencing the manhwa table
    author_id int NOT NULL, -- Foreign key referencing the authors table
    PRIMARY KEY (manhwa_id, author_id), -- Composite primary key to ensure uniqueness of the link
    FOREIGN KEY (manhwa_id) REFERENCES manhwa (manhwa_id) ON DELETE CASCADE, -- If a manhwa is deleted, remove its links here
    FOREIGN KEY (author_id) REFERENCES authors (author_id) ON DELETE CASCADE -- If an author is deleted, remove their links here
);

-- Create a linking table for the many-to-many relationship between manhwa and genres
-- A manhwa can have multiple genres, and a genre can apply to multiple manhwa
CREATE TABLE manhwa_genres (
    manhwa_id int NOT NULL, -- Foreign key referencing the manhwa table
    genre_id int NOT NULL, -- Foreign key referencing the genres table
    -- Add the 'weight' column here as you mentioned wanting to weight genres
    -- This column stores how much a manhwa leans into a specific genre (e.g., 0.7 for 70%)
    weight DECIMAL(3, 2) CHECK (weight >= 0.00 AND weight <= 1.00), -- Decimal value between 0.00 and 1.00
    PRIMARY KEY (manhwa_id, genre_id), -- Composite primary key to ensure uniqueness of the link
    FOREIGN KEY (manhwa_id) REFERENCES manhwa (manhwa_id) ON DELETE CASCADE, -- If a manhwa is deleted, remove its links here
    FOREIGN KEY (genre_id) REFERENCES genres (genre_id) ON DELETE CASCADE -- If a genre is deleted, remove its links here
);

-- You can add indexes later for performance, but this is a good start.
