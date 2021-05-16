INSERT INTO users_data (email)
VALUES('test@email.com');

INSERT INTO stock_list (user_id, "name")
VALUES(1, 'Test list');

INSERT INTO stock_items (list_id, "name", min_stock, max_stock)
VALUES(1, 'Test item', 1, 10);

INSERT INTO stock_movements (item_id, "date", quantity)
VALUES(1, '2021-05-10', 5);