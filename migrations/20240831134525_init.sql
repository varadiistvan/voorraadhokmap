-- Add migration script here

-- Create table for items
CREATE TABLE items (
    name TEXT NOT NULL PRIMARY KEY
);

-- Create table for item aliases
CREATE TABLE item_aliases (
    id INTEGER NOT NULL PRIMARY KEY,
    item_name TEXT NOT NULL,
    alias TEXT NOT NULL,
    FOREIGN KEY (item_name) REFERENCES items(name) ON UPDATE CASCADE ON DELETE CASCADE
    UNIQUE (item_name, alias)
);

-- Create table for columns with a composite primary key
CREATE TABLE columns (
    id INTEGER NOT NULL,  -- Index within each section
    section TEXT NOT NULL CHECK(section IN ('A', 'B', 'C', 'D', 'E', 'F', 'G', 'H')),
    PRIMARY KEY (section, id)
);

-- Create table for crate types
CREATE TABLE crate_types (
    width REAL NOT NULL,
    height REAL NOT NULL,
    name TEXT NOT NULL PRIMARY KEY
);

-- Create table for crates
CREATE TABLE crates (
    id INTEGER PRIMARY KEY,
    col_id INTEGER NOT NULL,
    rotation TEXT NOT NULL CHECK(rotation IN ('ShortSideWall', 'LongSideWall')),
    crate_type_name INTEGER NOT NULL,
    FOREIGN KEY (col_id) REFERENCES columns(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (crate_type_name) REFERENCES crate_types(name) ON UPDATE CASCADE
);

-- Create table for crate items (many-to-many relationship)
CREATE TABLE crates_items (
    id INTEGER PRIMARY KEY,
    crate_id INTEGER NOT NULL,
    item_name INTEGER NOT NULL,
    FOREIGN KEY (crate_id) REFERENCES crates(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (item_name) REFERENCES items(name) ON UPDATE CASCADE ON DELETE CASCADE
);
