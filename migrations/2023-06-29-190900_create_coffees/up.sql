CREATE TABLE coffees (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  manufacturer TEXT NOT NULL,
  country TEXT NOT NULL,
  processing TEXT NOT NULL,
  package INTEGER NOT NULL,
  price TEXT NOT NULL,
  url TEXT NOT NULL,
  image_url TEXT NOT NULL,
  available BOOLEAN NOT NULL DEFAULT FALSE
);
