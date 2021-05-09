CREATE KEYSPACE "stock"
    WITH replication = {'class': 'SimpleStrategy', 'replication_factor' : '3'};

CREATE TABLE stock.stock_movements (
    user int,
    item int,
    date date,
    difference counter,
    PRIMARY KEY (user, item, date)
);


UPDATE stock.stock_movements
SET difference = difference - 2
WHERE user = 1
  AND item = 1
  AND date = '2021-05-08';


SELECT *
FROM stock.stock_movements
WHERE user = 1
  AND item = 1
  AND date = '2021-05-08'