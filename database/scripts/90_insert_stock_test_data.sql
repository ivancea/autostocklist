INSERT INTO stock.item ("name", min_stock, max_stock, stock)
VALUES('Test item', 2, 10, 1);

INSERT INTO stock.item ("name", min_stock, max_stock, stock)
VALUES('No stock item', 1, 100, 0);


INSERT INTO stock.resupply (item_id, "date", quantity)
VALUES(1, '2021-05-10', 10);

INSERT INTO stock.loss (item_id, "date", quantity)
VALUES(1, '2021-05-12', 3);

INSERT INTO stock.loss (item_id, "date", quantity)
VALUES(1, '2021-05-14', 3);

INSERT INTO stock.loss (item_id, "date", quantity)
VALUES(1, '2021-05-16', 3);