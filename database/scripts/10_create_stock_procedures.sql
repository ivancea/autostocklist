CREATE PROCEDURE stock.update_stock_loss(
	item_id INTEGER,
	date DATE,
	quantity INTEGER
)
LANGUAGE SQL
AS $$
	-- Update item stock
	UPDATE stock.item
	SET stock = stock - quantity
	WHERE id = item_id;

	-- Add/Update stock loss historic
	INSERT INTO stock.loss (item_id, date, quantity)
    VALUES($1, $2, $3)
    ON CONFLICT (item_id, date)
    DO
    UPDATE SET quantity = stock.loss.quantity + excluded.quantity;
$$;

CREATE PROCEDURE stock.update_stock_resupply(
	item_id INTEGER,
	date DATE,
	quantity INTEGER
)
LANGUAGE SQL
AS $$
	-- Update item stock
	UPDATE stock.item
	SET stock = stock + quantity
	WHERE id = item_id;

	-- Add/Update stock resupply historic
	INSERT INTO stock.resupply (item_id, date, quantity)
    VALUES($1, $2, $3)
    ON CONFLICT (item_id, date)
    DO
    UPDATE SET quantity = stock.resupply.quantity + excluded.quantity;
$$;