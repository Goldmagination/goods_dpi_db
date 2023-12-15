CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    creation_time TIMESTAMP NOT NULL,
    description TEXT,
    address_id INTEGER NOT NULL REFERENCES addresses(id)
);
CREATE TABLE order_subcategories (
    id SERIAL PRIMARY KEY,
    order_id INTEGER NOT NULL REFERENCES orders(id),
    subcategory_id INTEGER NOT NULL REFERENCES subcategories(id)
);
