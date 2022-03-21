-- Your SQL goes here


CREATE TABLE orderslines (
  id SERIAL PRIMARY KEY,
  order_id references orders(id),
  product_name VARCHAR NOT NULL,
  product_id INTEGER NOT NULL,
  amount INTEGER NOT NULL,
)