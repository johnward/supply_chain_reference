-- Your SQL goes here

CREATE TABLE orders (
  id SERIAL PRIMARY KEY,
  customer_id INTEGER NOT NULL,
  address TEXT NOT NULL,
  fulfilled BOOLEAN NOT NULL DEFAULT 'f'
)