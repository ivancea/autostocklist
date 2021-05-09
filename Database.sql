CREATE KEYSPACE "stock"
    WITH replication = {'class': 'SimpleStrategy', 'replication_factor' : '3'};

CREATE TABLE stock.stock_movements (
    user int,
    item int,
    date date,
    change int,
    PRIMARY KEY (user, item, date)
);

INSERT INTO stock.stock_movements (user, item, date, change)
    VALUES (1, 1, '2021-05-08', -4);