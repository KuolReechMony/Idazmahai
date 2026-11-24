CREATE TABLE IF NOT EXISTS locations (
  -- `id` autoincrements cause of `INTEGER PRIMARY KEY`
  id INTEGER PRIMARY KEY,
  address TEXT NOT NULL CHECK (length(address) > 0),
  unit TEXT CHECK (length(unit) > 0) -- Can be NULL
) STRICT;
