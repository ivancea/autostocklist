CREATE TABLE stock_item (
    id INTEGER NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL,
    min_stock INTEGER CHECK (min_stock >= 0 OR min_stock IS NULL),
    max_stock INTEGER CHECK (max_stock > min_stock OR max_stock IS NULL)
);

CREATE TABLE stock_movement (
    item_id INTEGER NOT NULL,
    date DATE NOT NULL,
    quantity INTEGER NOT NULL,
    
    PRIMARY KEY (item_id, date),
    CONSTRAINT fk_item FOREIGN KEY (item_id) REFERENCES stock_item(id)
);

CREATE VIEW stock_total AS
    SELECT i.id AS item_id, SUM(COALESCE(m.quantity, 0)) AS stock, MAX(m.date) as last_movement_date
    FROM stock_item i
    LEFT JOIN stock_movement m
        ON i.id = m.item_id
    GROUP BY i.id;