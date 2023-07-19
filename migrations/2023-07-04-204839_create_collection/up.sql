CREATE TABLE collection (
  id INTEGER NOT NULL PRIMARY KEY,
  is_stored BOOLEAN NOT NULL,
  is_favorite BOOLEAN NOT NULL,
  amount_left INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  coffee_id INTEGER NOT NULL,
  FOREIGN KEY (coffee_id) REFERENCES coffees (id)
);
