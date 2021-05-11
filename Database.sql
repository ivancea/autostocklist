CREATE KEYSPACE "stock"
    WITH replication = {'class': 'SimpleStrategy', 'replication_factor' : '3'};

/*

Cambios:
¿date a timestamp?

Casos:
- Insertar cambio
- Coger cambios en un período


*/
CREATE TABLE stock.stock_movements (
    user int,
    item int,
    date date,
    amount counter,
    PRIMARY KEY ((user, item), date)
);

/*

Casos:
- Enumerar items de un usuario

*/
CREATE MATERIALIZED VIEW stock.stock_
AS
SELECT user, item, date
FROM stock.stock_movements 
PRIMARY KEY (user, item, date);


UPDATE stock.stock_movements
SET amount = amount - 2
WHERE user = 1
  AND item = 1
  AND date = '2021-05-08';


SELECT *
FROM stock.stock_movements
WHERE user = 1
  AND item = 1
  AND date = '2021-05-08'