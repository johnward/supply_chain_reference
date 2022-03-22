-- Your SQL goes here

CREATE TABLE orderlines (
  id SERIAL PRIMARY KEY,
  order_id SERIAL references orders,
  product_name VARCHAR NOT NULL,
  product_id INTEGER NOT NULL,
  amount INTEGER NOT NULL
)