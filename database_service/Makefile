DBCONTAINER=schro_schro-database_1
NETWORKNAME=schro_schro-net
EXPRESS=schro-mongo-express

inspect-compose:
	docker kill $(EXPRESS) || true
	docker rm $(EXPRESS) || true
	docker run -d --name $(EXPRESS) --network $(NETWORKNAME) -e ME_CONFIG_MONGODB_SERVER=$(DBCONTAINER) -p 8081:8081 mongo-express

compose-prebuilt:

build:
	docker build -t schrodat .

run:
	docker run --env-file ./database.env --env-file ./auth.env -p 27017:27017 schrodat 

run-detached:
	docker run --env-file ./database.env --env-file ./auth.env -p 27017:27017 -d schrodat 
