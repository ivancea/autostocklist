CREATE OR REPLACE PROCEDURE stock.update_stock_loss(
	_item_id INTEGER,
	_date DATE,
	_quantity INTEGER
)
LANGUAGE plpgsql
AS $$
DECLARE
BEGIN
	-- Update item stock
	UPDATE stock.item
	SET stock = stock - _quantity
	WHERE id = _item_id;

	IF _quantity < 0 THEN
		-- Update existing stock loss
		UPDATE stock.loss
		SET quantity = quantity + _quantity
		WHERE item_id = _item_id
			AND date = _date;
		
		IF NOT FOUND THEN
		     RAISE 'No loss entries for (%, %)', _item_id, _date;
		END IF;
	ELSE
		-- Add/Update stock loss
		INSERT INTO stock.loss (item_id, date, quantity)
	    VALUES(_item_id, _date, _quantity)
	    ON CONFLICT (item_id, date)
	    DO
	    UPDATE SET quantity = stock.loss.quantity + excluded.quantity;
	END IF;
END $$;

CREATE OR REPLACE PROCEDURE stock.update_stock_resupply(
	_item_id INTEGER,
	_date DATE,
	_quantity INTEGER
)
LANGUAGE plpgsql
AS $$
DECLARE
BEGIN
	-- Update item stock
	UPDATE stock.item
	SET stock = stock + _quantity
	WHERE id = _item_id;

	IF _quantity < 0 THEN
		-- Update existing stock resupply
		UPDATE stock.resupply
		SET quantity = quantity + _quantity
		WHERE item_id = _item_id
			AND date = _date;
		
		IF NOT FOUND THEN
		     RAISE 'No loss resupply for (%, %)', _item_id, _date;
		END IF;
	ELSE
		-- Add/Update stock resupply
		INSERT INTO stock.resupply (item_id, date, quantity)
	    VALUES(_item_id, _date, _quantity)
	    ON CONFLICT (item_id, date)
	    DO UPDATE
		SET quantity = stock.resupply.quantity + excluded.quantity;
	END IF;
END $$;