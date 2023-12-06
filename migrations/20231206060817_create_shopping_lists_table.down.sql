DROP TABLE IF EXISTS items;
DROP TRIGGER IF EXISTS updated_at_trigger_items;

DROP TABLE IF EXISTS shopping_lists;
DROP TRIGGER IF EXISTS updated_at_trigger_shopping_lists;

DROP FUNCTION IF EXISTS updated_at_trigger();
