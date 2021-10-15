CREATE SCHEMA stock;

CREATE TABLE stock.item (
    id INTEGER NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL,
    min_stock INTEGER NOT NULL CHECK (min_stock >= 0),
    max_stock INTEGER CHECK (max_stock > min_stock OR max_stock IS NULL),
    stock INTEGER CHECK (stock >= 0)
);

CREATE TABLE stock.loss (
    item_id INTEGER NOT NULL,
    date DATE NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity >= 0),
    
    PRIMARY KEY (item_id, date),
    CONSTRAINT fk_item FOREIGN KEY (item_id) REFERENCES stock.item(id)
);

CREATE TABLE stock.resupply (
    item_id INTEGER NOT NULL,
    date DATE NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity >= 0),
    
    PRIMARY KEY (item_id, date),
    CONSTRAINT fk_item FOREIGN KEY (item_id) REFERENCES stock.item(id)
);