INSERT INTO stock_item ("name", min_stock, max_stock)
VALUES('Test item', 2, 10);

INSERT INTO stock_item ("name", min_stock, max_stock)
VALUES('No stock item', 1, 100);


INSERT INTO stock_resupply (item_id, "date", quantity)
VALUES(1, '2021-05-10', 10);

INSERT INTO stock_loss (item_id, "date", quantity)
VALUES(1, '2021-05-12', 3);

INSERT INTO stock_loss (item_id, "date", quantity)
VALUES(1, '2021-05-14', 3);

INSERT INTO stock_loss (item_id, "date", quantity)
VALUES(1, '2021-05-16', 3);