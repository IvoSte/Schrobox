kubectl exec -it mongos-router-0 -c mongos-container bash
mongo

db.getSiblingDB('admin').auth("main_admin", "abc123");
sh.enableSharding("GaarkeukenDB");
sh.shardCollection("GaarkeukenDB.ledgers", {"groupId": 1}, true);
use GaarkeukenDB;
db.ledgers.insert({"groupId": "g0-xzc43", "groupName": "gaarkeukenTester"});
db.ledgers.find();
sh.status();




docker run --name mong -d -p 27017:27017 mongo:latest 

docker start mong


docker exec -it mong bash
mongo
db.getSiblingDB("admin").createUser({user:"main_admin",pwd:"abc123",roles:[{role:"root",db:"admin"}]});
db.getSiblingDB('admin').auth("main_admin", "abc123");
use GaarkeukenDB
db.createCollection("ledgers")
db.ledgers.createIndex({"groupId": 1}, true);
exit
exit
