conn = new Mongo();
db = conn.getDB("rinha");


db.clientes.createIndex({ "id": 1 }, { unique: true });

db.clientes.insert({ "nome": "o barato sai caro", limite: 1000 * 100, id: 1});
db.clientes.insert({ "nome": "zan corp ltda", limite: 800 * 100, id: 2});
db.clientes.insert({ "nome": "les cruders", limite: 10000 * 100, id: 3});
db.clientes.insert({ "nome": "padaria joia de cocaia", limite: 100000 * 100, id: 4});
db.clientes.insert({ "nome": "kid mais", limite: 5000 * 100, id: 5});
