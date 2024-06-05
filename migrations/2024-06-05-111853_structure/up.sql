CREATE TABLE customers (
  id INTEGER NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  email TEXT NOT NULL,
  description TEXT NOT NULL
);

INSERT INTO customers VALUES (0, "Testig Customer", "example@example.com", "Testing customer");

CREATE TABLE payments (
  id INTEGER NOT NULL PRIMARY KEY,
  created INTEGER NOT NULL, 
  amount INTEGER NOT NULL,
  currency TEXT NOT NULL CHECK(currency IN ('usd')), -- move to external table 
  "status" TEXT NOT NULL CHECK("status" IN ('wait', 'done', 'canceled')),
  method_type TEXT NOT NULL CHECK(method_type IN ('card')),
  customer_id INTEGER NOT NULL REFERENCES customers
);

CREATE TABLE charge (
  id INTEGER NOT NULL PRIMARY KEY,
  amount REAL NOT NULL,
  created INTEGER NOT NULL,
  payment_id INTEGER NOT NULL REFERENCES payments
);