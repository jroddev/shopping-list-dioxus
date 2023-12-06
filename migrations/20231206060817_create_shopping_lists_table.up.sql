
-- for uuid_generate_v4
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- function to refresh updated_at fields on change
CREATE  FUNCTION refresh_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = clock_timestamp();
    RETURN NEW;
END;
$$ language 'plpgsql';

---

CREATE TABLE shopping_lists (
    id uuid DEFAULT uuid_generate_v4 (),
    name TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);

CREATE TRIGGER updated_at_trigger_shopping_lists
    BEFORE UPDATE ON shopping_lists FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at();

---

CREATE TABLE items (
    id uuid DEFAULT uuid_generate_v4 (),
    parent_id uuid  REFERENCES shopping_lists(id) ON DELETE CASCADE,
    name TEXT,
    crossed BOOLEAN,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);


CREATE TRIGGER updated_at_trigger_items
    BEFORE UPDATE ON items FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at();



