CREATE TABLE payments (
  id INTEGER NOT NULL PRIMARY KEY,
  created INTEGER NOT NULL,
  amount INTEGER NOT NULL,
  currency TEXT NOT NULL,
  method_type TEXT NOT NULL,
  status TEXT NOT NULL
)