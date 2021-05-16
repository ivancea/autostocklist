CREATE TABLE users_data (
	id INTEGER NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	email TEXT NOT NULL UNIQUE
);

CREATE TABLE stock_list (
	id INTEGER NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id INTEGER NOT NULL,
	name TEXT NOT NULL,
    
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users_data(id)
);

CREATE TABLE stock_items (
    id INTEGER NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    list_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    min_stock INTEGER CHECK (min_stock >= 0 OR min_stock IS NULL),
    max_stock INTEGER CHECK (max_stock > min_stock OR max_stock IS NULL),
    
    CONSTRAINT fk_list FOREIGN KEY (list_id) REFERENCES stock_list(id)
);

CREATE TABLE stock_movements (
    item_id INTEGER NOT NULL,
    date DATE NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    
    PRIMARY KEY (item_id, date),
    CONSTRAINT fk_item FOREIGN KEY (item_id) REFERENCES stock_items(id)
);