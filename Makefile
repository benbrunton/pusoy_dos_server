
docker-build:
	docker build -t benbrunton/pusoy_dos .

compile:
	docker run --rm -v ${PWD}/project:/project benbrunton/pusoy_dos 


docker-server-build:
	docker build -t benbrunton/pd_server -f ./Dockerfile_run .

docker-server-run:
	docker run -d --name pd_server -P benbrunton/pd_server "./target/debug/pd_server"

docker-release: 
	cd project && make css js
	docker exec -t pd_server cargo build --release

docker-stop:
	docker stop pd_server

docker-restart:
	docker restart pd_server

docker-rm:
	docker rm pd_server

sh:
	docker exec -it pd_server bash

tail-logs:
	docker logs -f pd_server

reset-db:
	docker exec pd_server ./scripts/reset-db

deploy: 
	sh .deploy
