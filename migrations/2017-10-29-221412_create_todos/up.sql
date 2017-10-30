-- Your SQL goes here
CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    item_order INTEGER NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 'f'
)
